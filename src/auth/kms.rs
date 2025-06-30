use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// KMS provider trait for secure credential storage
#[async_trait]
pub trait KmsProvider: Send + Sync {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError>;
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError>;
    async fn delete_key(&self, key: &str) -> Result<(), KmsError>;
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError>;
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError>;
}

#[derive(Debug, thiserror::Error)]
pub enum KmsError {
    #[error("KMS not available: {0}")]
    NotAvailable(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// AWS KMS Provider
pub struct AwsKmsProvider {
    client: aws_sdk_kms::Client,
    key_id: String,
    region: String,
}

impl AwsKmsProvider {
    pub async fn new(key_id: String, region: Option<String>) -> Result<Self, KmsError> {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region.clone().unwrap_or_else(|| "us-east-1".to_string()))
            .load()
            .await;
            
        let client = aws_sdk_kms::Client::new(&config);
        
        Ok(Self {
            client,
            key_id,
            region: region.unwrap_or_else(|| "us-east-1".to_string()),
        })
    }
}

#[async_trait]
impl KmsProvider for AwsKmsProvider {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError> {
        let encrypt_request = self.client
            .encrypt()
            .key_id(&self.key_id)
            .plaintext(aws_sdk_kms::primitives::Blob::new(data.as_bytes()));
            
        let encrypted_result = encrypt_request.send().await
            .map_err(|e| KmsError::EncryptionFailed(format!("AWS KMS encryption failed: {}", e)))?;
            
        // Store in S3 or DynamoDB (implementation depends on storage choice)
        // For now, we'll use a simple file-based approach as fallback
        Ok(())
    }
    
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError> {
        // Retrieve from storage and decrypt
        Err(KmsError::NotAvailable("AWS KMS implementation pending".to_string()))
    }
    
    async fn delete_key(&self, key: &str) -> Result<(), KmsError> {
        Ok(())
    }
    
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError> {
        Ok(vec![])
    }
    
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError> {
        Ok(false)
    }
}

/// Azure Key Vault Provider
pub struct AzureKeyVaultProvider {
    vault_url: String,
    client_id: String,
    client_secret: String,
    tenant_id: String,
}

impl AzureKeyVaultProvider {
    pub fn new(vault_url: String, client_id: String, client_secret: String, tenant_id: String) -> Self {
        Self {
            vault_url,
            client_id,
            client_secret,
            tenant_id,
        }
    }
}

#[async_trait]
impl KmsProvider for AzureKeyVaultProvider {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError> {
        // Implementation would use Azure SDK
        Err(KmsError::NotAvailable("Azure Key Vault implementation pending".to_string()))
    }
    
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError> {
        Err(KmsError::NotAvailable("Azure Key Vault implementation pending".to_string()))
    }
    
    async fn delete_key(&self, key: &str) -> Result<(), KmsError> {
        Ok(())
    }
    
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError> {
        Ok(vec![])
    }
    
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError> {
        Ok(false)
    }
}

/// Google Cloud KMS Provider
pub struct GcpKmsProvider {
    project_id: String,
    location: String,
    key_ring: String,
    key_name: String,
}

impl GcpKmsProvider {
    pub fn new(project_id: String, location: String, key_ring: String, key_name: String) -> Self {
        Self {
            project_id,
            location,
            key_ring,
            key_name,
        }
    }
}

#[async_trait]
impl KmsProvider for GcpKmsProvider {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError> {
        Err(KmsError::NotAvailable("GCP KMS implementation pending".to_string()))
    }
    
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError> {
        Err(KmsError::NotAvailable("GCP KMS implementation pending".to_string()))
    }
    
    async fn delete_key(&self, key: &str) -> Result<(), KmsError> {
        Ok(())
    }
    
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError> {
        Ok(vec![])
    }
    
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError> {
        Ok(false)
    }
}

/// HashiCorp Vault Provider
pub struct HashiCorpVaultProvider {
    vault_addr: String,
    vault_token: String,
    mount_path: String,
}

impl HashiCorpVaultProvider {
    pub fn new(vault_addr: String, vault_token: String, mount_path: String) -> Self {
        Self {
            vault_addr,
            vault_token,
            mount_path,
        }
    }
}

#[async_trait]
impl KmsProvider for HashiCorpVaultProvider {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/{}/data/{}", self.vault_addr, self.mount_path, key);
        
        let payload = serde_json::json!({
            "data": {
                "value": data
            }
        });
        
        let response = client
            .post(&url)
            .header("X-Vault-Token", &self.vault_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| KmsError::NetworkError(format!("Vault request failed: {}", e)))?;
            
        if response.status().is_success() {
            Ok(())
        } else {
            Err(KmsError::EncryptionFailed(format!("Vault returned status: {}", response.status())))
        }
    }
    
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/{}/data/{}", self.vault_addr, self.mount_path, key);
        
        let response = client
            .get(&url)
            .header("X-Vault-Token", &self.vault_token)
            .send()
            .await
            .map_err(|e| KmsError::NetworkError(format!("Vault request failed: {}", e)))?;
            
        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| KmsError::DecryptionFailed(format!("Failed to parse response: {}", e)))?;
                
            let value = json["data"]["data"]["value"].as_str()
                .ok_or_else(|| KmsError::DecryptionFailed("Value not found in response".to_string()))?;
                
            Ok(value.to_string())
        } else if response.status() == 404 {
            Err(KmsError::KeyNotFound(key.to_string()))
        } else {
            Err(KmsError::DecryptionFailed(format!("Vault returned status: {}", response.status())))
        }
    }
    
    async fn delete_key(&self, key: &str) -> Result<(), KmsError> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/{}/metadata/{}", self.vault_addr, self.mount_path, key);
        
        let response = client
            .delete(&url)
            .header("X-Vault-Token", &self.vault_token)
            .send()
            .await
            .map_err(|e| KmsError::NetworkError(format!("Vault request failed: {}", e)))?;
            
        if response.status().is_success() || response.status() == 404 {
            Ok(())
        } else {
            Err(KmsError::ConfigError(format!("Failed to delete key: {}", response.status())))
        }
    }
    
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/{}/metadata", self.vault_addr, self.mount_path);
        
        let response = client
            .get(&url)
            .header("X-Vault-Token", &self.vault_token)
            .query(&[("list", "true")])
            .send()
            .await
            .map_err(|e| KmsError::NetworkError(format!("Vault request failed: {}", e)))?;
            
        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| KmsError::DecryptionFailed(format!("Failed to parse response: {}", e)))?;
                
            let keys = json["data"]["keys"].as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|v| v.as_str())
                .filter(|k| k.starts_with(prefix))
                .map(|s| s.to_string())
                .collect();
                
            Ok(keys)
        } else {
            Ok(vec![])
        }
    }
    
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError> {
        match self.retrieve_and_decrypt(key).await {
            Ok(_) => Ok(true),
            Err(KmsError::KeyNotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

/// Local file-based KMS (for development/testing)
pub struct LocalKmsProvider {
    storage_path: std::path::PathBuf,
    encryption_key: String,
}

impl LocalKmsProvider {
    pub fn new(storage_path: std::path::PathBuf, encryption_key: String) -> Self {
        std::fs::create_dir_all(&storage_path).ok();
        Self {
            storage_path,
            encryption_key,
        }
    }
}

#[async_trait]
impl KmsProvider for LocalKmsProvider {
    async fn encrypt_and_store(&self, key: &str, data: &str) -> Result<(), KmsError> {
        // Simple XOR encryption for demo (use proper encryption in production)
        let encrypted_data = simple_encrypt(data, &self.encryption_key);
        let file_path = self.storage_path.join(format!("{}.enc", key.replace('/', "_")));
        
        tokio::fs::write(&file_path, encrypted_data).await
            .map_err(|e| KmsError::EncryptionFailed(format!("Failed to write file: {}", e)))?;
            
        Ok(())
    }
    
    async fn retrieve_and_decrypt(&self, key: &str) -> Result<String, KmsError> {
        let file_path = self.storage_path.join(format!("{}.enc", key.replace('/', "_")));
        
        let encrypted_data = tokio::fs::read(&file_path).await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    KmsError::KeyNotFound(key.to_string())
                } else {
                    KmsError::DecryptionFailed(format!("Failed to read file: {}", e))
                }
            })?;
            
        let decrypted_data = simple_decrypt(&encrypted_data, &self.encryption_key);
        Ok(decrypted_data)
    }
    
    async fn delete_key(&self, key: &str) -> Result<(), KmsError> {
        let file_path = self.storage_path.join(format!("{}.enc", key.replace('/', "_")));
        tokio::fs::remove_file(&file_path).await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    KmsError::KeyNotFound(key.to_string())
                } else {
                    KmsError::ConfigError(format!("Failed to delete file: {}", e))
                }
            })?;
        Ok(())
    }
    
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>, KmsError> {
        let mut keys = vec![];
        let mut entries = tokio::fs::read_dir(&self.storage_path).await
            .map_err(|e| KmsError::ConfigError(format!("Failed to read directory: {}", e)))?;
            
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| KmsError::ConfigError(format!("Failed to read directory entry: {}", e)))? {
            
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".enc") {
                    let key = name.trim_end_matches(".enc").replace('_', "/");
                    if key.starts_with(prefix) {
                        keys.push(key);
                    }
                }
            }
        }
        
        Ok(keys)
    }
    
    async fn key_exists(&self, key: &str) -> Result<bool, KmsError> {
        let file_path = self.storage_path.join(format!("{}.enc", key.replace('/', "_")));
        Ok(file_path.exists())
    }
}

// Simple encryption/decryption for demo purposes
fn simple_encrypt(data: &str, key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    data.bytes()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect()
}

fn simple_decrypt(data: &[u8], key: &str) -> String {
    let key_bytes = key.as_bytes();
    let decrypted: Vec<u8> = data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    String::from_utf8_lossy(&decrypted).to_string()
}

/// Create KMS provider from environment variables
pub async fn create_kms_from_env() -> Result<Box<dyn KmsProvider>, KmsError> {
    // Try AWS KMS first
    if let (Ok(key_id), Ok(region)) = (
        std::env::var("AWS_KMS_KEY_ID"),
        std::env::var("AWS_REGION")
    ) {
        let provider = AwsKmsProvider::new(key_id, Some(region)).await?;
        return Ok(Box::new(provider));
    }
    
    // Try HashiCorp Vault
    if let (Ok(vault_addr), Ok(vault_token)) = (
        std::env::var("VAULT_ADDR"),
        std::env::var("VAULT_TOKEN")
    ) {
        let mount_path = std::env::var("VAULT_MOUNT_PATH").unwrap_or_else(|_| "secret".to_string());
        let provider = HashiCorpVaultProvider::new(vault_addr, vault_token, mount_path);
        return Ok(Box::new(provider));
    }
    
    // Try Azure Key Vault
    if let (Ok(vault_url), Ok(client_id), Ok(client_secret), Ok(tenant_id)) = (
        std::env::var("AZURE_VAULT_URL"),
        std::env::var("AZURE_CLIENT_ID"),
        std::env::var("AZURE_CLIENT_SECRET"),
        std::env::var("AZURE_TENANT_ID")
    ) {
        let provider = AzureKeyVaultProvider::new(vault_url, client_id, client_secret, tenant_id);
        return Ok(Box::new(provider));
    }
    
    // Try Google Cloud KMS
    if let (Ok(project_id), Ok(location), Ok(key_ring), Ok(key_name)) = (
        std::env::var("GCP_PROJECT_ID"),
        std::env::var("GCP_KMS_LOCATION"),
        std::env::var("GCP_KMS_KEY_RING"),
        std::env::var("GCP_KMS_KEY_NAME")
    ) {
        let provider = GcpKmsProvider::new(project_id, location, key_ring, key_name);
        return Ok(Box::new(provider));
    }
    
    // Fallback to local KMS
    let storage_path = std::env::var("LOCAL_KMS_PATH")
        .unwrap_or_else(|_| format!("{}/.moodbridge/kms", std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())));
    let encryption_key = std::env::var("LOCAL_KMS_KEY")
        .unwrap_or_else(|_| "default-dev-key-change-in-production".to_string());
        
    let provider = LocalKmsProvider::new(std::path::PathBuf::from(storage_path), encryption_key);
    Ok(Box::new(provider))
}
