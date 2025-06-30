//! # Enterprise Platform Integration Framework
//! 
//! This module provides the core traits and types for integrating with enterprise platforms
//! including Salesforce, AWS, Azure, Snowflake, and ETL systems.

use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use uuid::Uuid;

pub mod auth;
pub mod config;
pub mod salesforce;
pub mod aws;
pub mod azure; 
pub mod snowflake;
pub mod etl;
pub mod metrics;

/// Core integration result type
pub type IntegrationResult<T> = Result<T, IntegrationError>;

/// Unique identifier for integration operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OperationId(pub Uuid);

impl OperationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Platform integration capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationCapability {
    /// Data synchronization (read/write)
    DataSync,
    /// File storage and retrieval
    FileStorage,
    /// Real-time event processing
    EventProcessing,
    /// Analytics and reporting
    Analytics,
    /// Authentication and authorization
    Authentication,
    /// Serverless function execution
    ServerlessCompute,
    /// Message queuing and processing
    MessageProcessing,
    /// Data warehousing
    DataWarehouse,
    /// ETL pipeline execution
    ETLProcessing,
}

/// Platform connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Connection is healthy and operational
    Healthy,
    /// Connection has issues but is operational
    Degraded { reason: String },
    /// Connection is unhealthy
    Unhealthy { error: String },
    /// Connection is being established
    Connecting,
    /// Connection is not configured
    NotConfigured,
}

/// Integration metadata and health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHealth {
    pub platform_name: String,
    pub status: ConnectionStatus,
    pub last_checked: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
    pub capabilities: Vec<IntegrationCapability>,
    pub rate_limit_remaining: Option<u32>,
    pub rate_limit_reset: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Data synchronization operation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    pub operation_id: OperationId,
    pub platform: String,
    pub operation_type: SyncOperationType,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub records_processed: Option<u64>,
    pub errors: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperationType {
    Create,
    Read,
    Update,
    Delete,
    BulkSync,
    DeltaSync,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed { error: String },
    PartiallyCompleted { warnings: Vec<String> },
}

/// Core platform integration trait
#[async_trait]
pub trait PlatformIntegration: Send + Sync {
    /// Get the platform name (e.g., "salesforce", "aws", "azure")
    fn platform_name(&self) -> &'static str;

    /// Get supported capabilities for this integration
    fn capabilities(&self) -> Vec<IntegrationCapability>;

    /// Check the health and connectivity of the platform
    async fn health_check(&self) -> IntegrationResult<IntegrationHealth>;

    /// Initialize the integration with configuration
    async fn initialize(&mut self, config: &IntegrationConfig) -> IntegrationResult<()>;

    /// Shutdown and cleanup resources
    async fn shutdown(&mut self) -> IntegrationResult<()>;

    /// Authenticate with the platform (if required)
    async fn authenticate(&mut self) -> IntegrationResult<AuthenticationResult>;

    /// Refresh authentication credentials (if applicable)
    async fn refresh_auth(&mut self) -> IntegrationResult<AuthenticationResult>;
}

/// Data synchronization trait for platforms that support data operations
#[async_trait]
pub trait DataSyncIntegration: PlatformIntegration {
    /// Synchronize a single entity with the platform
    async fn sync_entity(
        &self,
        operation: SyncOperationType,
        entity_type: &str,
        entity_data: serde_json::Value,
    ) -> IntegrationResult<SyncOperation>;

    /// Perform bulk synchronization of multiple entities
    async fn bulk_sync(
        &self,
        operation: SyncOperationType,
        entity_type: &str,
        entities: Vec<serde_json::Value>,
    ) -> IntegrationResult<SyncOperation>;

    /// Perform delta synchronization based on timestamps
    async fn delta_sync(
        &self,
        entity_type: &str,
        last_sync: DateTime<Utc>,
    ) -> IntegrationResult<SyncOperation>;

    /// Query entities from the platform
    async fn query_entities(
        &self,
        entity_type: &str,
        query: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> IntegrationResult<Vec<serde_json::Value>>;
}

/// File storage trait for platforms that support file operations
#[async_trait]
pub trait FileStorageIntegration: PlatformIntegration {
    /// Upload a file to the platform
    async fn upload_file(
        &self,
        file_path: &str,
        content: Vec<u8>,
        metadata: HashMap<String, String>,
    ) -> IntegrationResult<FileUploadResult>;

    /// Download a file from the platform
    async fn download_file(&self, file_id: &str) -> IntegrationResult<FileDownloadResult>;

    /// Delete a file from the platform
    async fn delete_file(&self, file_id: &str) -> IntegrationResult<()>;

    /// List files with optional filtering
    async fn list_files(
        &self,
        prefix: Option<&str>,
        limit: Option<u32>,
    ) -> IntegrationResult<Vec<FileMetadata>>;
}

/// Event processing trait for real-time updates
#[async_trait]
pub trait EventProcessingIntegration: PlatformIntegration {
    /// Subscribe to real-time events from the platform
    async fn subscribe_to_events(
        &self,
        event_types: Vec<String>,
        callback: Box<dyn EventHandler>,
    ) -> IntegrationResult<SubscriptionId>;

    /// Unsubscribe from events
    async fn unsubscribe(&self, subscription_id: &SubscriptionId) -> IntegrationResult<()>;

    /// Send an event to the platform
    async fn send_event(&self, event: PlatformEvent) -> IntegrationResult<()>;
}

/// Analytics and reporting trait
#[async_trait]
pub trait AnalyticsIntegration: PlatformIntegration {
    /// Execute an analytics query
    async fn execute_query(
        &self,
        query: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> IntegrationResult<QueryResult>;

    /// Generate a report
    async fn generate_report(
        &self,
        report_type: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> IntegrationResult<ReportResult>;

    /// Get analytics metrics
    async fn get_metrics(
        &self,
        metric_names: Vec<String>,
        time_range: TimeRange,
    ) -> IntegrationResult<Vec<MetricValue>>;
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub platform: String,
    pub enabled: bool,
    pub credentials: CredentialConfig,
    pub connection: ConnectionConfig,
    pub features: FeatureConfig,
    pub rate_limiting: RateLimitConfig,
    pub retry_policy: RetryPolicyConfig,
    pub timeout: Duration,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialConfig {
    pub auth_type: AuthenticationType,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub tenant_id: Option<String>,
    pub private_key_path: Option<String>,
    pub token_endpoint: Option<String>,
    pub scope: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationType {
    OAuth2,
    ServiceAccount,
    ApiKey,
    Basic,
    Certificate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub base_url: String,
    pub api_version: Option<String>,
    pub region: Option<String>,
    pub pool_size: Option<u32>,
    pub keep_alive: bool,
    pub use_tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub enable_sync: bool,
    pub enable_real_time: bool,
    pub enable_analytics: bool,
    pub enable_file_storage: bool,
    pub batch_size: u32,
    pub sync_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: Option<u32>,
    pub requests_per_hour: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub burst_capacity: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicyConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub retry_on_timeout: bool,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub token_type: Option<String>,
    pub scope: Option<Vec<String>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// File operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResult {
    pub file_id: String,
    pub url: Option<String>,
    pub size_bytes: u64,
    pub checksum: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadResult {
    pub content: Vec<u8>,
    pub metadata: FileMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_id: String,
    pub name: String,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub checksum: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Event processing types
pub type SubscriptionId = String;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: PlatformEvent) -> IntegrationResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformEvent {
    pub event_id: String,
    pub event_type: String,
    pub platform: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Analytics types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub total_rows: Option<u64>,
    pub execution_time_ms: u64,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResult {
    pub report_id: String,
    pub report_type: String,
    pub generated_at: DateTime<Utc>,
    pub format: String,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub unit: Option<String>,
    pub labels: HashMap<String, String>,
}

/// Integration error types
#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Connection error: {message}")]
    ConnectionError { message: String },
    
    #[error("API error: {status_code} - {message}")]
    ApiError { status_code: u16, message: String },
    
    #[error("Rate limit exceeded: {retry_after:?}")]
    RateLimitExceeded { retry_after: Option<Duration> },
    
    #[error("Timeout error: operation took longer than {timeout:?}")]
    TimeoutError { timeout: Duration },
    
    #[error("Data validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
    
    #[error("Feature not supported: {feature}")]
    FeatureNotSupported { feature: String },
}

impl IntegrationError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            IntegrationError::NetworkError(_) => true,
            IntegrationError::TimeoutError { .. } => true,
            IntegrationError::ConnectionError { .. } => true,
            IntegrationError::ApiError { status_code, .. } => {
                // Retry on 5xx errors and some 4xx errors
                *status_code >= 500 || *status_code == 429 || *status_code == 408
            }
            IntegrationError::RateLimitExceeded { .. } => true,
            _ => false,
        }
    }

    /// Get suggested retry delay
    pub fn retry_delay(&self) -> Option<Duration> {
        match self {
            IntegrationError::RateLimitExceeded { retry_after } => *retry_after,
            IntegrationError::TimeoutError { .. } => Some(Duration::from_secs(5)),
            IntegrationError::ApiError { status_code, .. } if *status_code >= 500 => {
                Some(Duration::from_secs(2))
            }
            _ => None,
        }
    }
}

/// Integration manager for coordinating multiple platform integrations
pub struct IntegrationManager {
    integrations: HashMap<String, Box<dyn PlatformIntegration>>,
    config: HashMap<String, IntegrationConfig>,
}

impl IntegrationManager {
    pub fn new() -> Self {
        Self {
            integrations: HashMap::new(),
            config: HashMap::new(),
        }
    }

    /// Register a platform integration
    pub fn register_integration(
        &mut self,
        platform: String,
        integration: Box<dyn PlatformIntegration>,
        config: IntegrationConfig,
    ) {
        self.integrations.insert(platform.clone(), integration);
        self.config.insert(platform, config);
    }

    /// Get an integration by platform name
    pub fn get_integration(&self, platform: &str) -> Option<&dyn PlatformIntegration> {
        self.integrations.get(platform).map(|i| i.as_ref())
    }

    /// Initialize all registered integrations
    pub async fn initialize_all(&mut self) -> IntegrationResult<()> {
        for (platform, integration) in &mut self.integrations {
            if let Some(config) = self.config.get(platform) {
                if config.enabled {
                    integration.initialize(config).await?;
                }
            }
        }
        Ok(())
    }

    /// Get health status for all integrations
    pub async fn get_all_health(&self) -> HashMap<String, IntegrationResult<IntegrationHealth>> {
        let mut results = HashMap::new();
        
        for (platform, integration) in &self.integrations {
            let health = integration.health_check().await;
            results.insert(platform.clone(), health);
        }
        
        results
    }

    /// Shutdown all integrations
    pub async fn shutdown_all(&mut self) -> IntegrationResult<()> {
        for integration in self.integrations.values_mut() {
            let _ = integration.shutdown().await; // Continue even if some fail
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_id_generation() {
        let id1 = OperationId::new();
        let id2 = OperationId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_integration_error_retryable() {
        let retryable_error = IntegrationError::NetworkError(
            reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"))
        );
        assert!(retryable_error.is_retryable());

        let non_retryable_error = IntegrationError::ValidationError {
            message: "Invalid data".to_string(),
        };
        assert!(!non_retryable_error.is_retryable());
    }

    #[test]
    fn test_integration_config_serialization() {
        let config = IntegrationConfig {
            platform: "test".to_string(),
            enabled: true,
            credentials: CredentialConfig {
                auth_type: AuthenticationType::OAuth2,
                client_id: Some("test_client".to_string()),
                client_secret: Some("secret".to_string()),
                tenant_id: None,
                private_key_path: None,
                token_endpoint: None,
                scope: None,
            },
            connection: ConnectionConfig {
                base_url: "https://api.example.com".to_string(),
                api_version: Some("v1".to_string()),
                region: None,
                pool_size: Some(10),
                keep_alive: true,
                use_tls: true,
            },
            features: FeatureConfig {
                enable_sync: true,
                enable_real_time: false,
                enable_analytics: true,
                enable_file_storage: false,
                batch_size: 100,
                sync_interval_seconds: 300,
            },
            rate_limiting: RateLimitConfig {
                requests_per_minute: Some(60),
                requests_per_hour: Some(1000),
                requests_per_day: None,
                burst_capacity: Some(10),
            },
            retry_policy: RetryPolicyConfig {
                max_retries: 3,
                initial_delay_ms: 1000,
                max_delay_ms: 30000,
                backoff_multiplier: 2.0,
                retry_on_timeout: true,
            },
            timeout: Duration::from_secs(30),
            custom_settings: HashMap::new(),
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: IntegrationConfig = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config.platform, deserialized.platform);
        assert_eq!(config.enabled, deserialized.enabled);
    }
}
