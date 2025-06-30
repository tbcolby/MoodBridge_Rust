use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ring::{aead, digest, hmac, pbkdf2, rand};
use ring::rand::SecureRandom;
use base64::{Engine as _, engine::general_purpose};

pub mod encryption;
pub mod schema;
pub mod storage;
pub mod index;
pub mod query;
pub mod audit;

/// Core cryptographic database engine
pub struct CryptoDatabase {
    storage: Arc<dyn StorageEngine>,
    encryption: Arc<EncryptionManager>,
    indexing: Arc<IndexManager>,
    audit: Arc<AuditLogger>,
    config: DatabaseConfig,
}

/// Database configuration with cryptographic settings
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub master_key_id: String,
    pub encryption_algorithm: EncryptionAlgorithm,
    pub hash_algorithm: HashAlgorithm,
    pub key_derivation_rounds: u32,
    pub enable_compression: bool,
    pub enable_versioning: bool,
    pub audit_level: AuditLevel,
    pub max_record_size: usize,
    pub cache_size: usize,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            master_key_id: "default".to_string(),
            encryption_algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            hash_algorithm: HashAlgorithm::Sha256,
            key_derivation_rounds: 100_000,
            enable_compression: true,
            enable_versioning: true,
            audit_level: AuditLevel::Full,
            max_record_size: 10 * 1024 * 1024, // 10MB
            cache_size: 100 * 1024 * 1024,      // 100MB
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EncryptionAlgorithm {
    ChaCha20Poly1305,
    Aes256Gcm,
}

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
}

#[derive(Debug, Clone, Copy)]
pub enum AuditLevel {
    None,
    Basic,
    Full,
}

/// Encrypted record structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedRecord {
    pub id: Uuid,
    pub table_name: String,
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mac: Vec<u8>,
    pub version: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Database operation result
pub type DbResult<T> = Result<T, DatabaseError>;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Index error: {0}")]
    Index(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Authorization error: {0}")]
    Authorization(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
}

/// Storage engine trait for different backends
#[async_trait]
pub trait StorageEngine: Send + Sync {
    async fn store(&self, key: &str, value: &[u8]) -> DbResult<()>;
    async fn retrieve(&self, key: &str) -> DbResult<Vec<u8>>;
    async fn delete(&self, key: &str) -> DbResult<()>;
    async fn list_keys(&self, prefix: &str) -> DbResult<Vec<String>>;
    async fn batch_store(&self, entries: Vec<(String, Vec<u8>)>) -> DbResult<()>;
    async fn transaction<F, R>(&self, op: F) -> DbResult<R>
    where
        F: FnOnce() -> DbResult<R> + Send,
        R: Send;
}

impl CryptoDatabase {
    pub async fn new(
        storage: Arc<dyn StorageEngine>,
        config: DatabaseConfig,
    ) -> DbResult<Self> {
        let encryption = Arc::new(EncryptionManager::new(&config).await?);
        let indexing = Arc::new(IndexManager::new(&config).await?);
        let audit = Arc::new(AuditLogger::new(&config).await?);

        Ok(Self {
            storage,
            encryption,
            indexing,
            audit,
            config,
        })
    }

    /// Insert a new record
    pub async fn insert<T>(&self, table: &str, data: &T) -> DbResult<Uuid>
    where
        T: Serialize + Send + Sync,
    {
        let id = Uuid::new_v4();
        let serialized = serde_json::to_vec(data)
            .map_err(|e| DatabaseError::Validation(e.to_string()))?;

        // Encrypt the data
        let encrypted = self.encryption.encrypt(&serialized).await?;
        
        // Create record
        let record = EncryptedRecord {
            id,
            table_name: table.to_string(),
            encrypted_data: encrypted.ciphertext,
            nonce: encrypted.nonce,
            mac: encrypted.mac,
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };

        // Store record
        let key = format!("{}:{}", table, id);
        let record_bytes = serde_json::to_vec(&record)
            .map_err(|e| DatabaseError::Storage(e.to_string()))?;
        
        self.storage.store(&key, &record_bytes).await?;

        // Update indexes
        self.indexing.add_record(table, &id, data).await?;

        // Audit log
        self.audit.log_insert(table, &id).await?;

        Ok(id)
    }

    /// Retrieve a record by ID
    pub async fn get<T>(&self, table: &str, id: &Uuid) -> DbResult<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let key = format!("{}:{}", table, id);
        let record_bytes = self.storage.retrieve(&key).await?;
        
        let record: EncryptedRecord = serde_json::from_slice(&record_bytes)
            .map_err(|e| DatabaseError::Storage(e.to_string()))?;

        // Decrypt the data
        let encrypted = EncryptedData {
            ciphertext: record.encrypted_data,
            nonce: record.nonce,
            mac: record.mac,
        };
        
        let decrypted = self.encryption.decrypt(&encrypted).await?;
        
        let data: T = serde_json::from_slice(&decrypted)
            .map_err(|e| DatabaseError::Decryption(e.to_string()))?;

        // Audit log
        self.audit.log_read(table, id).await?;

        Ok(data)
    }

    /// Update an existing record
    pub async fn update<T>(&self, table: &str, id: &Uuid, data: &T) -> DbResult<()>
    where
        T: Serialize + Send + Sync,
    {
        let key = format!("{}:{}", table, id);
        
        // Retrieve existing record for version control
        let existing_bytes = self.storage.retrieve(&key).await?;
        let mut record: EncryptedRecord = serde_json::from_slice(&existing_bytes)
            .map_err(|e| DatabaseError::Storage(e.to_string()))?;

        // Serialize and encrypt new data
        let serialized = serde_json::to_vec(data)
            .map_err(|e| DatabaseError::Validation(e.to_string()))?;
        let encrypted = self.encryption.encrypt(&serialized).await?;

        // Update record
        record.encrypted_data = encrypted.ciphertext;
        record.nonce = encrypted.nonce;
        record.mac = encrypted.mac;
        record.version += 1;
        record.updated_at = Utc::now();

        // Store updated record
        let record_bytes = serde_json::to_vec(&record)
            .map_err(|e| DatabaseError::Storage(e.to_string()))?;
        
        self.storage.store(&key, &record_bytes).await?;

        // Update indexes
        self.indexing.update_record(table, id, data).await?;

        // Audit log
        self.audit.log_update(table, id).await?;

        Ok(())
    }

    /// Delete a record
    pub async fn delete(&self, table: &str, id: &Uuid) -> DbResult<()> {
        let key = format!("{}:{}", table, id);
        
        // Remove from storage
        self.storage.delete(&key).await?;

        // Remove from indexes
        self.indexing.remove_record(table, id).await?;

        // Audit log
        self.audit.log_delete(table, id).await?;

        Ok(())
    }

    /// Query records with encrypted search
    pub async fn query<T>(&self, query: &QueryBuilder) -> DbResult<Vec<T>>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let record_ids = self.indexing.search(query).await?;
        let mut results = Vec::new();

        for id in record_ids {
            match self.get::<T>(&query.table, &id).await {
                Ok(record) => results.push(record),
                Err(DatabaseError::NotFound(_)) => continue, // Skip deleted records
                Err(e) => return Err(e),
            }
        }

        // Audit log
        self.audit.log_query(&query.table, results.len()).await?;

        Ok(results)
    }

    /// Get database statistics
    pub async fn stats(&self) -> DbResult<DatabaseStats> {
        let tables = self.list_tables().await?;
        let mut stats = DatabaseStats::default();

        for table in tables {
            let keys = self.storage.list_keys(&format!("{}:", table)).await?;
            stats.table_counts.insert(table, keys.len());
            stats.total_records += keys.len();
        }

        Ok(stats)
    }

    /// List all tables
    pub async fn list_tables(&self) -> DbResult<Vec<String>> {
        let all_keys = self.storage.list_keys("").await?;
        let mut tables = std::collections::HashSet::new();

        for key in all_keys {
            if let Some(table) = key.split(':').next() {
                tables.insert(table.to_string());
            }
        }

        Ok(tables.into_iter().collect())
    }
}

#[derive(Debug, Default)]
pub struct DatabaseStats {
    pub total_records: usize,
    pub table_counts: HashMap<String, usize>,
}

/// Encrypted data container
#[derive(Debug, Clone)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub mac: Vec<u8>,
}

/// Query builder for encrypted searches
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    pub table: String,
    pub conditions: Vec<QueryCondition>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: Option<String>,
    pub order_desc: bool,
}

#[derive(Debug, Clone)]
pub struct QueryCondition {
    pub field: String,
    pub operator: QueryOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum QueryOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            conditions: Vec::new(),
            limit: None,
            offset: None,
            order_by: None,
            order_desc: false,
        }
    }

    pub fn where_eq(mut self, field: &str, value: serde_json::Value) -> Self {
        self.conditions.push(QueryCondition {
            field: field.to_string(),
            operator: QueryOperator::Equals,
            value,
        });
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(mut self, field: &str, desc: bool) -> Self {
        self.order_by = Some(field.to_string());
        self.order_desc = desc;
        self
    }
}
