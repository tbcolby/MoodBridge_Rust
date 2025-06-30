use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit}};
use ring::digest::{Context, SHA256};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::AppError;
use crate::import_wizard::classifier::SensitivityLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionStandard {
    AES256GCM,      // For HighlyConfidential/TopSecret
    AES128GCM,      // For Confidential
    ChaCha20Poly1305, // Alternative modern encryption
    None,           // For Public/Internal
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub standard: EncryptionStandard,
    pub key_derivation_iterations: u32,
    pub compress_before_encrypt: bool,
    pub verify_integrity: bool,
    pub secure_delete_original: bool,
}

impl EncryptionConfig {
    pub fn from_sensitivity(sensitivity: &SensitivityLevel) -> Self {
        match sensitivity {
            SensitivityLevel::TopSecret => EncryptionConfig {
                standard: EncryptionStandard::AES256GCM,
                key_derivation_iterations: 100_000,
                compress_before_encrypt: true,
                verify_integrity: true,
                secure_delete_original: true,
            },
            SensitivityLevel::HighlyConfidential => EncryptionConfig {
                standard: EncryptionStandard::AES256GCM,
                key_derivation_iterations: 50_000,
                compress_before_encrypt: true,
                verify_integrity: true,
                secure_delete_original: true,
            },
            SensitivityLevel::Confidential => EncryptionConfig {
                standard: EncryptionStandard::AES128GCM,
                key_derivation_iterations: 25_000,
                compress_before_encrypt: false,
                verify_integrity: true,
                secure_delete_original: false,
            },
            SensitivityLevel::Internal => EncryptionConfig {
                standard: EncryptionStandard::None,
                key_derivation_iterations: 0,
                compress_before_encrypt: false,
                verify_integrity: false,
                secure_delete_original: false,
            },
            SensitivityLevel::Public => EncryptionConfig {
                standard: EncryptionStandard::None,
                key_derivation_iterations: 0,
                compress_before_encrypt: false,
                verify_integrity: false,
                secure_delete_original: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedFile {
    pub original_path: String,
    pub encrypted_path: String,
    pub encryption_config: EncryptionConfig,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
    pub checksum_original: String,
    pub checksum_encrypted: String,
    pub file_size_original: u64,
    pub file_size_encrypted: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct FileEncryptor {
    config: EncryptionConfig,
}

impl FileEncryptor {
    pub fn new(config: EncryptionConfig) -> Self {
        Self { config }
    }

    pub async fn encrypt_file(
        &self,
        input_path: &str,
        output_path: &str,
        password: &str,
    ) -> Result<EncryptedFile, AppError> {
        match self.config.standard {
            EncryptionStandard::None => {
                // Just copy the file
                fs::copy(input_path, output_path)
                    .map_err(|e| AppError::InternalError(format!("Failed to copy file: {}", e)))?;
                
                let metadata = fs::metadata(input_path)
                    .map_err(|e| AppError::InternalError(format!("Failed to read metadata: {}", e)))?;
                
                let checksum = self.calculate_checksum(input_path)?;
                
                Ok(EncryptedFile {
                    original_path: input_path.to_string(),
                    encrypted_path: output_path.to_string(),
                    encryption_config: self.config.clone(),
                    salt: vec![],
                    nonce: vec![],
                    checksum_original: checksum.clone(),
                    checksum_encrypted: checksum,
                    file_size_original: metadata.len(),
                    file_size_encrypted: metadata.len(),
                    created_at: chrono::Utc::now(),
                })
            }
            EncryptionStandard::AES256GCM | EncryptionStandard::AES128GCM => {
                self.encrypt_with_aes_gcm(input_path, output_path, password).await
            }
            EncryptionStandard::ChaCha20Poly1305 => {
                // Future implementation
                Err(AppError::InternalError("ChaCha20Poly1305 not yet implemented".to_string()))
            }
        }
    }

    async fn encrypt_with_aes_gcm(
        &self,
        input_path: &str,
        output_path: &str,
        password: &str,
    ) -> Result<EncryptedFile, AppError> {
        // Generate random salt and nonce
        let salt = self.generate_salt();
        let nonce_bytes = self.generate_nonce();
        
        // Derive encryption key from password
        let key = self.derive_key(password, &salt)?;
        
        // Read input file
        let plaintext = fs::read(input_path)
            .map_err(|e| AppError::InternalError(format!("Failed to read input file: {}", e)))?;
        
        // Compress if configured
        let data_to_encrypt = if self.config.compress_before_encrypt {
            self.compress_data(&plaintext)?
        } else {
            plaintext
        };
        
        // Calculate original checksum
        let checksum_original = self.calculate_checksum(input_path)?;
        
        // Encrypt the data
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher
            .encrypt(nonce, data_to_encrypt.as_ref())
            .map_err(|e| AppError::InternalError(format!("Encryption failed: {}", e)))?;
        
        // Prepare encrypted file format: salt + nonce + ciphertext
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&salt);
        encrypted_data.extend_from_slice(&nonce_bytes);
        encrypted_data.extend_from_slice(&ciphertext);
        
        // Write encrypted file
        fs::write(output_path, &encrypted_data)
            .map_err(|e| AppError::InternalError(format!("Failed to write encrypted file: {}", e)))?;
        
        // Calculate encrypted file checksum
        let checksum_encrypted = self.calculate_checksum(output_path)?;
        
        // Get file sizes
        let file_size_original = fs::metadata(input_path)
            .map_err(|e| AppError::InternalError(format!("Failed to read original metadata: {}", e)))?
            .len();
        
        let file_size_encrypted = encrypted_data.len() as u64;
        
        // Secure delete original if configured
        if self.config.secure_delete_original {
            self.secure_delete(input_path)?;
        }
        
        Ok(EncryptedFile {
            original_path: input_path.to_string(),
            encrypted_path: output_path.to_string(),
            encryption_config: self.config.clone(),
            salt,
            nonce: nonce_bytes,
            checksum_original,
            checksum_encrypted,
            file_size_original,
            file_size_encrypted,
            created_at: chrono::Utc::now(),
        })
    }

    pub async fn decrypt_file(
        &self,
        encrypted_file: &EncryptedFile,
        output_path: &str,
        password: &str,
    ) -> Result<(), AppError> {
        match encrypted_file.encryption_config.standard {
            EncryptionStandard::None => {
                fs::copy(&encrypted_file.encrypted_path, output_path)
                    .map_err(|e| AppError::InternalError(format!("Failed to copy file: {}", e)))?;
                Ok(())
            }
            EncryptionStandard::AES256GCM | EncryptionStandard::AES128GCM => {
                self.decrypt_with_aes_gcm(encrypted_file, output_path, password).await
            }
            EncryptionStandard::ChaCha20Poly1305 => {
                Err(AppError::InternalError("ChaCha20Poly1305 not yet implemented".to_string()))
            }
        }
    }

    async fn decrypt_with_aes_gcm(
        &self,
        encrypted_file: &EncryptedFile,
        output_path: &str,
        password: &str,
    ) -> Result<(), AppError> {
        // Read encrypted file
        let encrypted_data = fs::read(&encrypted_file.encrypted_path)
            .map_err(|e| AppError::InternalError(format!("Failed to read encrypted file: {}", e)))?;
        
        // Extract salt, nonce, and ciphertext
        if encrypted_data.len() < 32 + 12 { // minimum: 32 bytes salt + 12 bytes nonce
            return Err(AppError::ValidationError("Invalid encrypted file format".to_string()));
        }
        
        let salt = &encrypted_data[0..32];
        let nonce_bytes = &encrypted_data[32..44];
        let ciphertext = &encrypted_data[44..];
        
        // Derive decryption key
        let key = self.derive_key(password, salt)?;
        
        // Decrypt the data
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let decrypted_data = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::ValidationError(format!("Decryption failed: {}", e)))?;
        
        // Decompress if needed
        let final_data = if encrypted_file.encryption_config.compress_before_encrypt {
            self.decompress_data(&decrypted_data)?
        } else {
            decrypted_data
        };
        
        // Write decrypted file
        fs::write(output_path, &final_data)
            .map_err(|e| AppError::InternalError(format!("Failed to write decrypted file: {}", e)))?;
        
        // Verify integrity if configured
        if encrypted_file.encryption_config.verify_integrity {
            let checksum = self.calculate_checksum(output_path)?;
            if checksum != encrypted_file.checksum_original {
                return Err(AppError::ValidationError("File integrity check failed".to_string()));
            }
        }
        
        Ok(())
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>, AppError> {
        use ring::pbkdf2;
        
        let mut key_bytes = [0u8; 32]; // 256 bits for AES-256
        
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            std::num::NonZeroU32::new(self.config.key_derivation_iterations)
                .ok_or_else(|| AppError::InternalError("Invalid iteration count".to_string()))?,
            salt,
            password.as_bytes(),
            &mut key_bytes,
        );
        
        Ok(Key::<Aes256Gcm>::from_slice(&key_bytes).clone())
    }

    fn generate_salt(&self) -> Vec<u8> {
        let mut salt = vec![0u8; 32];
        rand::thread_rng().fill(&mut salt[..]);
        salt
    }

    fn generate_nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0u8; 12]; // 96 bits for GCM
        rand::thread_rng().fill(&mut nonce[..]);
        nonce
    }

    fn calculate_checksum(&self, file_path: &str) -> Result<String, AppError> {
        let mut context = Context::new(&SHA256);
        let data = fs::read(file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to read file for checksum: {}", e)))?;
        
        context.update(&data);
        let digest = context.finish();
        Ok(hex::encode(digest.as_ref()))
    }

    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, AppError> {
        // Simple compression using deflate
        use std::io::Write;
        use flate2::write::GzEncoder;
        use flate2::Compression;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| AppError::InternalError(format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| AppError::InternalError(format!("Compression finalization failed: {}", e)))
    }

    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>, AppError> {
        use std::io::Read;
        use flate2::read::GzDecoder;
        
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| AppError::InternalError(format!("Decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }

    fn secure_delete(&self, file_path: &str) -> Result<(), AppError> {
        // Secure deletion by overwriting with random data multiple times
        let path = Path::new(file_path);
        if !path.exists() {
            return Ok(());
        }
        
        let file_size = fs::metadata(file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to get file size: {}", e)))?
            .len();
        
        // Overwrite with random data 3 times
        for _ in 0..3 {
            let mut random_data = vec![0u8; file_size as usize];
            rand::thread_rng().fill(&mut random_data[..]);
            
            fs::write(file_path, &random_data)
                .map_err(|e| AppError::InternalError(format!("Failed to overwrite file: {}", e)))?;
        }
        
        // Finally delete the file
        fs::remove_file(file_path)
            .map_err(|e| AppError::InternalError(format!("Failed to delete file: {}", e)))?;
        
        Ok(())
    }
}
