// HeidiMaetl ETL Platform - Core Module
// Transforming MoodBridge_Rust into a world-class ETL platform

pub mod verb_engine;
pub mod verbs;
pub mod connectors;
pub mod transformers;
pub mod validators;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// HeidiMaetl - The heart of our ETL platform
/// Named after nobility (Heidi) and strength (Maetl/Metal)
#[derive(Debug, Clone)]
pub struct HeidiMaetl {
    pub engine: Arc<RwLock<verb_engine::VerbEngine>>,
    pub pipelines: Arc<RwLock<HashMap<Uuid, Pipeline>>>,
    pub config: EtlConfig,
}

/// Core data pipeline representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub verbs: Vec<VerbStep>,
    pub status: PipelineStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Individual verb execution step in a pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbStep {
    pub id: Uuid,
    pub verb_name: String,
    pub config: VerbConfig,
    pub dependencies: Vec<Uuid>,
    pub outputs: Vec<String>,
}

/// Configuration for verb execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbConfig {
    pub parameters: HashMap<String, serde_json::Value>,
    pub source: Option<DataSource>,
    pub destination: Option<DataDestination>,
    pub transformations: Vec<Transformation>,
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub source_type: SourceType,
    pub connection_string: String,
    pub schema: Option<DataSchema>,
    pub filters: Vec<String>,
}

/// Data destination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDestination {
    pub destination_type: DestinationType,
    pub connection_string: String,
    pub schema: Option<DataSchema>,
    pub options: HashMap<String, String>,
}

/// Data transformation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transform_type: TransformationType,
    pub source_field: String,
    pub target_field: String,
    pub expression: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Data schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    pub fields: Vec<SchemaField>,
    pub primary_key: Vec<String>,
    pub indexes: Vec<String>,
}

/// Individual field in data schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default_value: Option<serde_json::Value>,
    pub constraints: Vec<String>,
}

/// Pipeline execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineStatus {
    Draft,
    Scheduled,
    Running,
    Completed,
    Failed,
    Paused,
    Cancelled,
}

/// Supported data source types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Database,
    Api,
    File,
    Stream,
    Cloud,
    Custom(String),
}

/// Supported data destination types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DestinationType {
    Database,
    Api,
    File,
    Stream,
    Cloud,
    Cache,
    Custom(String),
}

/// Transformation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    Map,
    Filter,
    Aggregate,
    Join,
    Split,
    Merge,
    Validate,
    Cleanse,
    Custom(String),
}

/// Data types supported by HeidiMaetl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Json,
    Binary,
    Array(Box<DataType>),
    Custom(String),
}

/// ETL platform configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtlConfig {
    pub max_concurrent_pipelines: usize,
    pub default_timeout: u64,
    pub retry_attempts: u32,
    pub log_level: String,
    pub monitoring_enabled: bool,
    pub ai_features_enabled: bool,
}

impl Default for EtlConfig {
    fn default() -> Self {
        Self {
            max_concurrent_pipelines: 10,
            default_timeout: 3600, // 1 hour
            retry_attempts: 3,
            log_level: "info".to_string(),
            monitoring_enabled: true,
            ai_features_enabled: true,
        }
    }
}

impl HeidiMaetl {
    /// Create a new HeidiMaetl ETL platform instance
    pub async fn new(config: EtlConfig) -> Self {
        let engine = Arc::new(RwLock::new(verb_engine::VerbEngine::new()));
        let pipelines = Arc::new(RwLock::new(HashMap::new()));
        
        Self {
            engine,
            pipelines,
            config,
        }
    }

    /// Register a new pipeline
    pub async fn register_pipeline(&self, pipeline: Pipeline) -> Result<Uuid, Box<dyn std::error::Error>> {
        let mut pipelines = self.pipelines.write().await;
        let pipeline_id = pipeline.id;
        pipelines.insert(pipeline_id, pipeline);
        Ok(pipeline_id)
    }

    /// Execute a pipeline by ID
    pub async fn execute_pipeline(&self, pipeline_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let pipelines = self.pipelines.read().await;
        let pipeline = pipelines.get(&pipeline_id)
            .ok_or("Pipeline not found")?;
        
        let engine = self.engine.read().await;
        engine.execute_pipeline(pipeline).await
    }

    /// Get pipeline status
    pub async fn get_pipeline_status(&self, pipeline_id: Uuid) -> Option<PipelineStatus> {
        let pipelines = self.pipelines.read().await;
        pipelines.get(&pipeline_id).map(|p| p.status.clone())
    }

    /// List all pipelines
    pub async fn list_pipelines(&self) -> Vec<Pipeline> {
        let pipelines = self.pipelines.read().await;
        pipelines.values().cloned().collect()
    }
}

/// Core result type for ETL operations
pub type EtlResult<T> = Result<T, EtlError>;

/// ETL-specific error types
#[derive(Debug, thiserror::Error)]
pub enum EtlError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Transformation error: {0}")]
    Transformation(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Pipeline execution error: {0}")]
    Execution(String),
    
    #[error("Data quality error: {0}")]
    DataQuality(String),
    
    #[error("AI processing error: {0}")]
    AiProcessing(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heidi_maetl_creation() {
        let config = EtlConfig::default();
        let heidi = HeidiMaetl::new(config).await;
        
        assert_eq!(heidi.config.max_concurrent_pipelines, 10);
        assert!(heidi.config.ai_features_enabled);
    }

    #[tokio::test]
    async fn test_pipeline_registration() {
        let config = EtlConfig::default();
        let heidi = HeidiMaetl::new(config).await;
        
        let pipeline = Pipeline {
            id: Uuid::new_v4(),
            name: "Test Pipeline".to_string(),
            description: "A test pipeline".to_string(),
            verbs: vec![],
            status: PipelineStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        };
        
        let pipeline_id = pipeline.id;
        let result = heidi.register_pipeline(pipeline).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), pipeline_id);
    }
}
