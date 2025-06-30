use ring::{aead, digest, hmac, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::{Zeroize, ZeroizeOnDrop};
use crate::auth::kms::KmsProvider;
use super::{DatabaseConfig, DatabaseError, DbResult, EncryptedData, EncryptionAlgorithm};

/// Encryption manager for handling all cryptographic operations
pub struct EncryptionManager {
    keys: Arc<RwLock<HashMap<String, DerivedKey>>>,
    kms: Arc<dyn KmsProvider>,
    config: DatabaseConfig,
    rng: Arc<dyn SecureRandom>,
}

/// A derived encryption key with metadata
#[derive(Clone, ZeroizeOnDrop)]
struct DerivedKey {
    #[zeroize(skip)]
    algorithm: EncryptionAlgorithm,
    key: Vec<u8>,
    #[zeroize(skip)]
    created_at: chrono::DateTime<chrono::Utc>,
    #[zeroize(skip)]
    version: u32,
}

/// Key derivation parameters
#[derive(Debug, Clone)]
pub struct KeyDerivationParams {
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub algorithm: pbkdf2::Algorithm,
}

impl EncryptionManager {
    pub async fn new(config: &DatabaseConfig) -> DbResult<Self> {
        let kms = crate::auth::kms::create_kms_from_env().await
            .map_err(|e| DatabaseError::Encryption(format!("KMS initialization failed: {}", e)))?;

        let rng = Arc::new(rand::SystemRandom::new());
        
        Ok(Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            kms,
            config: config.clone(),
            rng,
        })
    }

    /// Encrypt data using the configured algorithm
    pub async fn encrypt(&self, plaintext: &[u8]) -> DbResult<EncryptedData> {
        let key = self.get_or_derive_key(&self.config.master_key_id).await?;
        
        match self.config.encryption_algorithm {
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                self.encrypt_chacha20_poly1305(plaintext, &key.key).await
            }
            EncryptionAlgorithm::Aes256Gcm => {
                self.encrypt_aes256_gcm(plaintext, &key.key).await
            }
        }
    }

    /// Decrypt data using the configured algorithm
    pub async fn decrypt(&self, encrypted: &EncryptedData) -> DbResult<Vec<u8>> {
        let key = self.get_or_derive_key(&self.config.master_key_id).await?;
        
        match self.config.encryption_algorithm {
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                self.decrypt_chacha20_poly1305(encrypted, &key.key).await
            }
            EncryptionAlgorithm::Aes256Gcm => {
                self.decrypt_aes256_gcm(encrypted, &key.key).await
            }
        }
    }

    /// Generate a new encryption key and store it in KMS
    pub async fn generate_key(&self, key_id: &str) -> DbResult<()> {
        let mut key_bytes = vec![0u8; 32]; // 256-bit key
        self.rng.fill(&mut key_bytes)
            .map_err(|e| DatabaseError::Encryption(format!("Key generation failed: {:?}", e)))?;

        // Store the raw key in KMS
        let key_b64 = base64::engine::general_purpose::STANDARD.encode(&key_bytes);
        self.kms.encrypt_and_store(key_id, &key_b64).await
            .map_err(|e| DatabaseError::Encryption(format!("KMS storage failed: {}", e)))?;

        // Clear the key from memory
        key_bytes.zeroize();

        Ok(())
    }

    /// Rotate an existing key
    pub async fn rotate_key(&self, key_id: &str) -> DbResult<()> {
        // Generate new key
        self.generate_key(key_id).await?;
        
        // Remove old key from cache
        let mut keys = self.keys.write().await;
        keys.remove(key_id);
        
        Ok(())
    }

    /// Get or derive a key for encryption/decryption
    async fn get_or_derive_key(&self, key_id: &str) -> DbResult<DerivedKey> {
        // Check cache first
        {
            let keys = self.keys.read().await;
            if let Some(key) = keys.get(key_id) {
                return Ok(key.clone());
            }
        }

        // Derive key if not in cache
        let derived_key = self.derive_key(key_id).await?;
        
        // Cache the derived key
        {
            let mut keys = self.keys.write().await;
            keys.insert(key_id.to_string(), derived_key.clone());
        }

        Ok(derived_key)
    }

    /// Derive a key from the master key stored in KMS
    async fn derive_key(&self, key_id: &str) -> DbResult<DerivedKey> {
        // Retrieve master key from KMS
        let master_key_b64 = self.kms.retrieve_and_decrypt(key_id).await
            .map_err(|e| DatabaseError::Encryption(format!("KMS retrieval failed: {}", e)))?;

        let master_key = base64::engine::general_purpose::STANDARD.decode(&master_key_b64)
            .map_err(|e| DatabaseError::Encryption(format!("Base64 decode failed: {}", e)))?;

        // Generate salt for key derivation
        let mut salt = vec![0u8; 32];
        self.rng.fill(&mut salt)
            .map_err(|e| DatabaseError::Encryption(format!("Salt generation failed: {:?}", e)))?;

        // Derive encryption key using PBKDF2
        let mut derived = vec![0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(self.config.key_derivation_rounds).unwrap(),
            &salt,
            &master_key,
            &mut derived,
        );

        Ok(DerivedKey {
            algorithm: self.config.encryption_algorithm,
            key: derived,
            created_at: chrono::Utc::now(),
            version: 1,
        })
    }

    /// Encrypt using ChaCha20-Poly1305
    async fn encrypt_chacha20_poly1305(&self, plaintext: &[u8], key: &[u8]) -> DbResult<EncryptedData> {
        let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, key)
            .map_err(|e| DatabaseError::Encryption(format!("Key creation failed: {:?}", e)))?;

        let mut nonce_bytes = vec![0u8; 12]; // ChaCha20-Poly1305 uses 12-byte nonces
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| DatabaseError::Encryption(format!("Nonce generation failed: {:?}", e)))?;

        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes.clone())
            .map_err(|e| DatabaseError::Encryption(format!("Nonce creation failed: {:?}", e)))?;

        let sealing_key = aead::LessSafeKey::new(unbound_key);
        
        let mut ciphertext = plaintext.to_vec();
        let tag = sealing_key.seal_in_place_detached(nonce, aead::Aad::empty(), &mut ciphertext)
            .map_err(|e| DatabaseError::Encryption(format!("Encryption failed: {:?}", e)))?;

        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes,
            mac: tag.as_ref().to_vec(),
        })
    }

    /// Decrypt using ChaCha20-Poly1305
    async fn decrypt_chacha20_poly1305(&self, encrypted: &EncryptedData, key: &[u8]) -> DbResult<Vec<u8>> {
        let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, key)
            .map_err(|e| DatabaseError::Decryption(format!("Key creation failed: {:?}", e)))?;

        let nonce = aead::Nonce::try_assume_unique_for_key(encrypted.nonce.clone())
            .map_err(|e| DatabaseError::Decryption(format!("Nonce creation failed: {:?}", e)))?;

        let opening_key = aead::LessSafeKey::new(unbound_key);
        
        let mut ciphertext_with_tag = encrypted.ciphertext.clone();
        ciphertext_with_tag.extend_from_slice(&encrypted.mac);

        let plaintext = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut ciphertext_with_tag)
            .map_err(|e| DatabaseError::Decryption(format!("Decryption failed: {:?}", e)))?;

        Ok(plaintext.to_vec())
    }

    /// Encrypt using AES-256-GCM
    async fn encrypt_aes256_gcm(&self, plaintext: &[u8], key: &[u8]) -> DbResult<EncryptedData> {
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| DatabaseError::Encryption(format!("Key creation failed: {:?}", e)))?;

        let mut nonce_bytes = vec![0u8; 12]; // AES-GCM uses 12-byte nonces
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| DatabaseError::Encryption(format!("Nonce generation failed: {:?}", e)))?;

        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes.clone())
            .map_err(|e| DatabaseError::Encryption(format!("Nonce creation failed: {:?}", e)))?;

        let sealing_key = aead::LessSafeKey::new(unbound_key);
        
        let mut ciphertext = plaintext.to_vec();
        let tag = sealing_key.seal_in_place_detached(nonce, aead::Aad::empty(), &mut ciphertext)
            .map_err(|e| DatabaseError::Encryption(format!("Encryption failed: {:?}", e)))?;

        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes,
            mac: tag.as_ref().to_vec(),
        })
    }

    /// Decrypt using AES-256-GCM
    async fn decrypt_aes256_gcm(&self, encrypted: &EncryptedData, key: &[u8]) -> DbResult<Vec<u8>> {
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
            .map_err(|e| DatabaseError::Decryption(format!("Key creation failed: {:?}", e)))?;

        let nonce = aead::Nonce::try_assume_unique_for_key(encrypted.nonce.clone())
            .map_err(|e| DatabaseError::Decryption(format!("Nonce creation failed: {:?}", e)))?;

        let opening_key = aead::LessSafeKey::new(unbound_key);
        
        let mut ciphertext_with_tag = encrypted.ciphertext.clone();
        ciphertext_with_tag.extend_from_slice(&encrypted.mac);

        let plaintext = opening_key.open_in_place(nonce, aead::Aad::empty(), &mut ciphertext_with_tag)
            .map_err(|e| DatabaseError::Decryption(format!("Decryption failed: {:?}", e)))?;

        Ok(plaintext.to_vec())
    }

    /// Generate HMAC for data integrity
    pub fn generate_hmac(&self, data: &[u8], key: &[u8]) -> DbResult<Vec<u8>> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        let signature = hmac::sign(&key, data);
        Ok(signature.as_ref().to_vec())
    }

    /// Verify HMAC for data integrity
    pub fn verify_hmac(&self, data: &[u8], mac: &[u8], key: &[u8]) -> DbResult<bool> {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key);
        match hmac::verify(&key, data, mac) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Hash data using the configured hash algorithm
    pub fn hash_data(&self, data: &[u8]) -> Vec<u8> {
        match self.config.hash_algorithm {
            super::HashAlgorithm::Sha256 => {
                digest::digest(&digest::SHA256, data).as_ref().to_vec()
            }
            super::HashAlgorithm::Sha512 => {
                digest::digest(&digest::SHA512, data).as_ref().to_vec()
            }
            super::HashAlgorithm::Blake3 => {
                // Note: Blake3 would require an additional dependency
                // For now, fallback to SHA256
                digest::digest(&digest::SHA256, data).as_ref().to_vec()
            }
        }
    }

    /// Clear all cached keys (for security)
    pub async fn clear_cache(&self) {
        let mut keys = self.keys.write().await;
        keys.clear();
    }
}

impl Drop for EncryptionManager {
    fn drop(&mut self) {
        // Ensure keys are zeroized when the manager is dropped
        // The ZeroizeOnDrop derive on DerivedKey will handle the actual zeroization
    }
}

/// Utility functions for secure random generation
pub struct SecureRandom;

impl SecureRandom {
    /// Generate a secure random UUID
    pub fn generate_uuid() -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }

    /// Generate secure random bytes
    pub fn generate_bytes(len: usize) -> DbResult<Vec<u8>> {
        let rng = rand::SystemRandom::new();
        let mut bytes = vec![0u8; len];
        rng.fill(&mut bytes)
            .map_err(|e| DatabaseError::Encryption(format!("Random generation failed: {:?}", e)))?;
        Ok(bytes)
    }

    /// Generate a secure random string (base64 encoded)
    pub fn generate_string(len: usize) -> DbResult<String> {
        let bytes = Self::generate_bytes(len)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_db::DatabaseConfig;

    #[tokio::test]
    async fn test_encryption_roundtrip() {
        let config = DatabaseConfig::default();
        let manager = EncryptionManager::new(&config).await.unwrap();
        
        // Generate a test key
        manager.generate_key("test_key").await.unwrap();
        
        let plaintext = b"Hello, encrypted world!";
        let encrypted = manager.encrypt(plaintext).await.unwrap();
        let decrypted = manager.decrypt(&encrypted).await.unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_hmac_verification() {
        let config = DatabaseConfig::default();
        let manager = EncryptionManager::new(&config).await.unwrap();
        
        let data = b"test data";
        let key = SecureRandom::generate_bytes(32).unwrap();
        
        let mac = manager.generate_hmac(data, &key).unwrap();
        assert!(manager.verify_hmac(data, &mac, &key).unwrap());
        
        // Test with wrong data
        let wrong_data = b"wrong data";
        assert!(!manager.verify_hmac(wrong_data, &mac, &key).unwrap());
    }

    #[test]
    fn test_secure_random() {
        let uuid = SecureRandom::generate_uuid();
        assert_ne!(uuid, uuid::Uuid::nil());
        
        let bytes = SecureRandom::generate_bytes(32).unwrap();
        assert_eq!(bytes.len(), 32);
        
        let string = SecureRandom::generate_string(32).unwrap();
        assert!(!string.is_empty());
    }
}
