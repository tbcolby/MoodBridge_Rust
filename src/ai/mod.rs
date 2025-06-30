pub mod llm;
pub mod patterns;
pub mod analytics;
pub mod fabric_integration;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// AI Insight types for legal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    Pattern,
    RiskAssessment,
    Recommendation,
    TimelineCorrelation,
    SentimentAnalysis,
    DocumentAnalysis,
}

/// AI-generated insight structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInsight {
    pub insight_type: InsightType,
    pub confidence_score: f64,
    pub data: serde_json::Value,
    pub generated_by: String,
    pub created_at: DateTime<Utc>,
}

/// Legal pattern detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    pub pattern_name: String,
    pub pattern_type: String,
    pub detection_criteria: HashMap<String, serde_json::Value>,
    pub severity_weight: f64,
    pub active: bool,
}

/// AI Analysis request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub operation_type: String,
    pub input_data: serde_json::Value,
    pub options: Option<HashMap<String, String>>,
}

/// AI Analysis response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub insights: Vec<AiInsight>,
    pub processing_time_ms: u128,
    pub model_used: String,
    pub error_message: Option<String>,
}

/// Main AI service trait
#[async_trait::async_trait]
pub trait AiService {
    async fn analyze_document(&self, content: &str, document_type: &str) -> Result<AnalysisResponse, AiError>;
    async fn detect_patterns(&self, data: &serde_json::Value) -> Result<Vec<AiInsight>, AiError>;
    async fn generate_timeline_events(&self, context: &str) -> Result<Vec<serde_json::Value>, AiError>;
    async fn assess_risk(&self, placement_denial: &serde_json::Value) -> Result<f64, AiError>;
    async fn analyze_communication_sentiment(&self, message: &str) -> Result<f64, AiError>;
}

/// AI service errors
#[derive(Debug, thiserror::Error)]
pub enum AiError {
    #[error("API error: {0}")]
    ApiError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Model error: {0}")]
    ModelError(String),
    #[error("Timeout error")]
    TimeoutError,
}

/// AI service configuration
#[derive(Debug, Clone)]
pub struct AiConfig {
    pub openai_api_key: Option<String>,
    pub openai_base_url: String,
    pub default_model: String,
    pub fabric_patterns_path: Option<String>,
    pub enable_fabric_integration: bool,
    pub max_retries: u32,
    pub timeout_seconds: u64,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            openai_base_url: "https://api.openai.com/v1".to_string(),
            default_model: "gpt-4".to_string(),
            fabric_patterns_path: std::env::var("FABRIC_PATTERNS_PATH").ok(),
            enable_fabric_integration: true,
            max_retries: 3,
            timeout_seconds: 30,
        }
    }
}
