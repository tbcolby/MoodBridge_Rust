use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::fs;
use serde::{Deserialize, Serialize};
use super::{StorageEngine, DatabaseError, DbResult};

/// File-based storage engine with compression and backup support
pub struct FileStorageEngine {
    root_path: PathBuf,
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    config: FileStorageConfig,
}

#[derive(Debug, Clone)]
pub struct FileStorageConfig {
    pub enable_compression: bool,
    pub enable_backup: bool,
    pub backup_interval: std::time::Duration,
    pub max_file_size: usize,
    pub cache_size: usize,
}

impl Default for FileStorageConfig {
    fn default() -> Self {
        Self {
            enable_compression: true,
            enable_backup: true,
            backup_interval: std::time::Duration::from_secs(3600), // 1 hour
            max_file_size: 100 * 1024 * 1024, // 100MB
            cache_size: 50 * 1024 * 1024,     // 50MB
        }
    }
}

impl FileStorageEngine {
    pub async fn new(root_path: PathBuf, config: FileStorageConfig) -> DbResult<Self> {
        // Create root directory if it doesn't exist
        fs::create_dir_all(&root_path).await
            .map_err(|e| DatabaseError::Storage(format!("Failed to create directory: {}", e)))?;

        // Create subdirectories for organization
        let data_dir = root_path.join("data");
        let backup_dir = root_path.join("backups");
        let index_dir = root_path.join("indexes");

        for dir in [&data_dir, &backup_dir, &index_dir] {
            fs::create_dir_all(dir).await
                .map_err(|e| DatabaseError::Storage(format!("Failed to create directory: {}", e)))?;
        }

        Ok(Self {
            root_path,
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Get the file path for a given key
    fn get_file_path(&self, key: &str) -> PathBuf {
        // Use hash-based directory structure to avoid filesystem limitations
        let hash = sha2::Sha256::digest(key.as_bytes());
        let hex_hash = hex::encode(&hash[..2]); // Use first 2 bytes for directory structure
        
        self.root_path
            .join("data")
            .join(&hex_hash[..2])
            .join(&hex_hash[2..4])
            .join(format!("{}.dat", hex::encode(hash)))
    }

    /// Compress data if compression is enabled
    fn compress_data(&self, data: &[u8]) -> DbResult<Vec<u8>> {
        if !self.config.enable_compression {
            return Ok(data.to_vec());
        }

        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| DatabaseError::Storage(format!("Compression failed: {}", e)))?;
        
        encoder.finish()
            .map_err(|e| DatabaseError::Storage(format!("Compression failed: {}", e)))
    }

    /// Decompress data if compression is enabled
    fn decompress_data(&self, data: &[u8]) -> DbResult<Vec<u8>> {
        if !self.config.enable_compression {
            return Ok(data.to_vec());
        }

        use flate2::read::GzDecoder;
        use std::io::Read;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| DatabaseError::Storage(format!("Decompression failed: {}", e)))?;
        
        Ok(decompressed)
    }

    /// Create backup of a file
    async fn backup_file(&self, key: &str, data: &[u8]) -> DbResult<()> {
        if !self.config.enable_backup {
            return Ok(());
        }

        let backup_path = self.root_path
            .join("backups")
            .join(format!("{}.backup", hex::encode(sha2::Sha256::digest(key.as_bytes()))));

        let backup_data = BackupEntry {
            key: key.to_string(),
            data: data.to_vec(),
            timestamp: chrono::Utc::now(),
        };

        let serialized = serde_json::to_vec(&backup_data)
            .map_err(|e| DatabaseError::Storage(format!("Backup serialization failed: {}", e)))?;

        fs::write(&backup_path, &serialized).await
            .map_err(|e| DatabaseError::Storage(format!("Backup write failed: {}", e)))?;

        Ok(())
    }

    /// Clean up old cache entries based on LRU
    async fn cleanup_cache(&self) {
        let mut cache = self.cache.write().await;
        let current_size: usize = cache.values().map(|v| v.len()).sum();
        
        if current_size > self.config.cache_size {
            // Simple cleanup: remove half the entries
            // In a production system, you'd implement proper LRU
            let keys_to_remove: Vec<String> = cache.keys().take(cache.len() / 2).cloned().collect();
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BackupEntry {
    key: String,
    data: Vec<u8>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
impl StorageEngine for FileStorageEngine {
    async fn store(&self, key: &str, value: &[u8]) -> DbResult<()> {
        // Compress data
        let compressed = self.compress_data(value)?;
        
        // Get file path and create parent directories
        let file_path = self.get_file_path(key);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| DatabaseError::Storage(format!("Failed to create directory: {}", e)))?;
        }

        // Write to file
        fs::write(&file_path, &compressed).await
            .map_err(|e| DatabaseError::Storage(format!("Failed to write file: {}", e)))?;

        // Create backup
        self.backup_file(key, value).await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), value.to_vec());
        }

        // Cleanup cache if needed
        self.cleanup_cache().await;

        Ok(())
    }

    async fn retrieve(&self, key: &str) -> DbResult<Vec<u8>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(data) = cache.get(key) {
                return Ok(data.clone());
            }
        }

        // Read from file
        let file_path = self.get_file_path(key);
        let compressed = fs::read(&file_path).await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    DatabaseError::NotFound(key.to_string())
                } else {
                    DatabaseError::Storage(format!("Failed to read file: {}", e))
                }
            })?;

        // Decompress data
        let data = self.decompress_data(&compressed)?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), data.clone());
        }

        Ok(data)
    }

    async fn delete(&self, key: &str) -> DbResult<()> {
        // Remove from cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(key);
        }

        // Delete file
        let file_path = self.get_file_path(key);
        if file_path.exists() {
            fs::remove_file(&file_path).await
                .map_err(|e| DatabaseError::Storage(format!("Failed to delete file: {}", e)))?;
        }

        Ok(())
    }

    async fn list_keys(&self, prefix: &str) -> DbResult<Vec<String>> {
        let mut keys = Vec::new();
        let data_dir = self.root_path.join("data");

        // Recursively walk directory structure
        let mut stack = vec![data_dir];
        
        while let Some(dir) = stack.pop() {
            let mut entries = fs::read_dir(&dir).await
                .map_err(|e| DatabaseError::Storage(format!("Failed to read directory: {}", e)))?;

            while let Some(entry) = entries.next_entry().await
                .map_err(|e| DatabaseError::Storage(format!("Failed to read directory entry: {}", e)))? {
                
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if let Some(file_name) = path.file_stem() {
                    if let Some(file_str) = file_name.to_str() {
                        if file_str.ends_with(".dat") {
                            // Extract key from filename (this is a simplified approach)
                            // In practice, you'd maintain a separate index
                            if let Some(key) = self.extract_key_from_file(&path).await? {
                                if key.starts_with(prefix) {
                                    keys.push(key);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(keys)
    }

    async fn batch_store(&self, entries: Vec<(String, Vec<u8>)>) -> DbResult<()> {
        // Process entries in parallel for better performance
        use futures::future::try_join_all;

        let futures = entries.into_iter().map(|(key, value)| {
            self.store(&key, &value)
        });

        try_join_all(futures).await?;
        Ok(())
    }

    async fn transaction<F, R>(&self, op: F) -> DbResult<R>
    where
        F: FnOnce() -> DbResult<R> + Send,
        R: Send,
    {
        // For file-based storage, we implement a simple transaction using temporary files
        // In a production system, you'd want proper ACID transactions
        
        // Execute the operation
        let result = op()?;
        
        // For now, we just return the result
        // In a full implementation, you'd maintain transaction logs
        Ok(result)
    }
}

impl FileStorageEngine {
    /// Extract the original key from a file (simplified implementation)
    async fn extract_key_from_file(&self, _path: &std::path::Path) -> DbResult<Option<String>> {
        // This is a simplified implementation
        // In practice, you'd maintain a separate index file that maps file hashes to keys
        // For now, return None to indicate we can't extract the key
        Ok(None)
    }
}

/// In-memory storage engine for testing and development
pub struct MemoryStorageEngine {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorageEngine {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for MemoryStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageEngine for MemoryStorageEngine {
    async fn store(&self, key: &str, value: &[u8]) -> DbResult<()> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> DbResult<Vec<u8>> {
        let data = self.data.read().await;
        data.get(key)
            .cloned()
            .ok_or_else(|| DatabaseError::NotFound(key.to_string()))
    }

    async fn delete(&self, key: &str) -> DbResult<()> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }

    async fn list_keys(&self, prefix: &str) -> DbResult<Vec<String>> {
        let data = self.data.read().await;
        let keys = data.keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        Ok(keys)
    }

    async fn batch_store(&self, entries: Vec<(String, Vec<u8>)>) -> DbResult<()> {
        let mut data = self.data.write().await;
        for (key, value) in entries {
            data.insert(key, value);
        }
        Ok(())
    }

    async fn transaction<F, R>(&self, op: F) -> DbResult<R>
    where
        F: FnOnce() -> DbResult<R> + Send,
        R: Send,
    {
        // For in-memory storage, transactions are atomic by nature
        op()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_storage_engine() {
        let temp_dir = TempDir::new().unwrap();
        let config = FileStorageConfig::default();
        let engine = FileStorageEngine::new(temp_dir.path().to_path_buf(), config).await.unwrap();

        // Test store and retrieve
        let key = "test_key";
        let value = b"test_value";
        
        engine.store(key, value).await.unwrap();
        let retrieved = engine.retrieve(key).await.unwrap();
        assert_eq!(value, retrieved.as_slice());

        // Test delete
        engine.delete(key).await.unwrap();
        assert!(engine.retrieve(key).await.is_err());
    }

    #[tokio::test]
    async fn test_memory_storage_engine() {
        let engine = MemoryStorageEngine::new();

        // Test store and retrieve
        let key = "test_key";
        let value = b"test_value";
        
        engine.store(key, value).await.unwrap();
        let retrieved = engine.retrieve(key).await.unwrap();
        assert_eq!(value, retrieved.as_slice());

        // Test delete
        engine.delete(key).await.unwrap();
        assert!(engine.retrieve(key).await.is_err());
    }

    #[tokio::test]
    async fn test_batch_operations() {
        let engine = MemoryStorageEngine::new();
        
        let entries = vec![
            ("key1".to_string(), b"value1".to_vec()),
            ("key2".to_string(), b"value2".to_vec()),
            ("key3".to_string(), b"value3".to_vec()),
        ];

        engine.batch_store(entries).await.unwrap();

        let value1 = engine.retrieve("key1").await.unwrap();
        let value2 = engine.retrieve("key2").await.unwrap();
        let value3 = engine.retrieve("key3").await.unwrap();

        assert_eq!(b"value1", value1.as_slice());
        assert_eq!(b"value2", value2.as_slice());
        assert_eq!(b"value3", value3.as_slice());
    }

    #[tokio::test]
    async fn test_list_keys() {
        let engine = MemoryStorageEngine::new();
        
        engine.store("user:1", b"data1").await.unwrap();
        engine.store("user:2", b"data2").await.unwrap();
        engine.store("post:1", b"data3").await.unwrap();

        let user_keys = engine.list_keys("user:").await.unwrap();
        assert_eq!(user_keys.len(), 2);
        assert!(user_keys.contains(&"user:1".to_string()));
        assert!(user_keys.contains(&"user:2".to_string()));

        let post_keys = engine.list_keys("post:").await.unwrap();
        assert_eq!(post_keys.len(), 1);
        assert!(post_keys.contains(&"post:1".to_string()));
    }
}
