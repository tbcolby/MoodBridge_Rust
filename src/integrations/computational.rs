//! # Computational Engine Integration Framework
//! 
//! This module provides a plugin architecture for integrating computational engines
//! like Wolfram Alpha, SymPy, MATLAB, Mathematica, and other mathematical/scientific
//! computation services.

use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use uuid::Uuid;

use super::{IntegrationResult, IntegrationError, IntegrationHealth, ConnectionStatus, PlatformIntegration};

/// Unique identifier for computational queries
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(pub Uuid);

impl QueryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Types of computational capabilities supported by engines
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComputationalCapability {
    /// Basic mathematical calculations
    BasicMath,
    /// Advanced mathematics (calculus, linear algebra, etc.)
    AdvancedMath,
    /// Statistical analysis and probability
    Statistics,
    /// Data analysis and visualization
    DataAnalysis,
    /// Physics calculations and simulations
    Physics,
    /// Chemistry calculations and molecular analysis
    Chemistry,
    /// Engineering calculations
    Engineering,
    /// Natural language mathematical queries
    NaturalLanguageQuery,
    /// Symbolic mathematics
    SymbolicMath,
    /// Numerical analysis
    NumericalAnalysis,
    /// Graph theory and discrete mathematics
    GraphTheory,
    /// Machine learning and AI computations
    MachineLearning,
    /// Financial mathematics
    FinancialMath,
    /// Unit conversion and dimensional analysis
    UnitConversion,
    /// Image and signal processing
    SignalProcessing,
}

/// Input format for computational queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryInputFormat {
    /// Natural language query (e.g., "integrate x^2 from 0 to 5")
    NaturalLanguage(String),
    /// Mathematical expression in specific notation
    Mathematical {
        expression: String,
        notation: MathNotation,
    },
    /// Structured query with parameters
    Structured {
        operation: String,
        parameters: HashMap<String, serde_json::Value>,
    },
    /// Code in engine-specific language
    Code {
        language: String,
        code: String,
    },
}

/// Mathematical notation systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MathNotation {
    /// LaTeX mathematical notation
    LaTeX,
    /// Wolfram Language syntax
    WolframLanguage,
    /// MATLAB syntax
    MATLAB,
    /// Python/SymPy syntax
    SymPy,
    /// Mathematica syntax
    Mathematica,
    /// Standard mathematical notation
    Standard,
}

/// Output format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Plain text result
    PlainText,
    /// LaTeX formatted output
    LaTeX,
    /// JSON structured data
    JSON,
    /// Image/plot (PNG, SVG, etc.)
    Image { format: String },
    /// HTML formatted output
    HTML,
    /// Markdown formatted output
    Markdown,
    /// Engine-specific format
    EngineSpecific { format: String },
}

/// Priority level for computational queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Computational query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationalQuery {
    pub query_id: QueryId,
    pub input: QueryInputFormat,
    pub capabilities_required: Vec<ComputationalCapability>,
    pub output_format: OutputFormat,
    pub priority: QueryPriority,
    pub timeout: Option<Duration>,
    pub context: Option<QueryContext>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Additional context for computational queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryContext {
    /// Previous queries in the same session
    pub session_history: Vec<QueryId>,
    /// Variables defined in the session
    pub variables: HashMap<String, serde_json::Value>,
    /// Units preference
    pub units: Option<String>,
    /// Precision requirements
    pub precision: Option<u32>,
    /// Domain-specific context
    pub domain: Option<String>,
}

/// Result of a computational query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationalResult {
    pub query_id: QueryId,
    pub engine_name: String,
    pub success: bool,
    pub result: Option<QueryOutput>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub cost: Option<QueryCost>,
    pub confidence: Option<f64>,
    pub alternatives: Vec<QueryOutput>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Output from a computational query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOutput {
    pub format: OutputFormat,
    pub content: serde_json::Value,
    pub description: Option<String>,
    pub visualization: Option<VisualizationData>,
    pub references: Vec<String>,
}

/// Visualization data for plots, graphs, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    pub type_: String,
    pub data: serde_json::Value,
    pub settings: HashMap<String, serde_json::Value>,
}

/// Cost information for computational queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCost {
    pub credits_used: Option<u32>,
    pub monetary_cost: Option<f64>,
    pub currency: Option<String>,
    pub rate_limit_consumed: Option<u32>,
}

/// Status of a computational query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryStatus {
    Pending,
    Processing,
    Completed,
    Failed { error: String },
    Timeout,
    RateLimited,
}

/// Core trait for computational engine plugins
#[async_trait]
pub trait ComputationalEngine: PlatformIntegration {
    /// Get the computational capabilities supported by this engine
    fn supported_capabilities(&self) -> Vec<ComputationalCapability>;
    
    /// Get the supported input formats
    fn supported_input_formats(&self) -> Vec<QueryInputFormat>;
    
    /// Get the supported output formats
    fn supported_output_formats(&self) -> Vec<OutputFormat>;
    
    /// Check if the engine can handle a specific query
    fn can_handle_query(&self, query: &ComputationalQuery) -> bool;
    
    /// Execute a computational query
    async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult>;
    
    /// Get the status of a running query (for async operations)
    async fn get_query_status(&self, query_id: &QueryId) -> IntegrationResult<QueryStatus>;
    
    /// Cancel a running query
    async fn cancel_query(&self, query_id: &QueryId) -> IntegrationResult<()>;
    
    /// Get usage statistics for the engine
    async fn get_usage_stats(&self) -> IntegrationResult<EngineUsageStats>;
    
    /// Validate a query before execution
    async fn validate_query(&self, query: &ComputationalQuery) -> IntegrationResult<ValidationResult>;
}

/// Usage statistics for a computational engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineUsageStats {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub average_execution_time_ms: f64,
    pub total_cost: Option<f64>,
    pub rate_limit_status: RateLimitStatus,
    pub last_query_time: Option<DateTime<Utc>>,
}

/// Rate limiting status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    pub requests_remaining: Option<u32>,
    pub reset_time: Option<DateTime<Utc>>,
    pub daily_limit: Option<u32>,
    pub monthly_limit: Option<u32>,
}

/// Query validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub estimated_cost: Option<QueryCost>,
    pub estimated_execution_time: Option<Duration>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Manager for coordinating multiple computational engines
pub struct ComputationalEngineManager {
    engines: HashMap<String, Box<dyn ComputationalEngine>>,
    routing_strategy: RoutingStrategy,
    fallback_chain: Vec<String>,
}

/// Strategy for routing queries to engines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    /// Route to the first engine that can handle the query
    FirstCapable,
    /// Route to the engine with the best capability match
    BestMatch,
    /// Route to the fastest engine for the query type
    Fastest,
    /// Route to the most cost-effective engine
    CostEffective,
    /// Route based on custom logic
    Custom,
}

impl ComputationalEngineManager {
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            engines: HashMap::new(),
            routing_strategy: strategy,
            fallback_chain: Vec::new(),
        }
    }
    
    /// Register a computational engine
    pub fn register_engine(&mut self, name: String, engine: Box<dyn ComputationalEngine>) {
        self.engines.insert(name, engine);
    }
    
    /// Set the fallback chain for engine selection
    pub fn set_fallback_chain(&mut self, chain: Vec<String>) {
        self.fallback_chain = chain;
    }
    
    /// Execute a query using the best available engine
    pub async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        let engine_name = self.select_engine(&query)?;
        
        if let Some(engine) = self.engines.get(&engine_name) {
            let mut result = engine.execute_query(query).await?;
            result.engine_name = engine_name;
            Ok(result)
        } else {
            Err(IntegrationError::InternalError {
                message: format!("Engine '{}' not found", engine_name),
            })
        }
    }
    
    /// Execute a query with fallback to other engines if the primary fails
    pub async fn execute_query_with_fallback(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        let primary_engine = self.select_engine(&query)?;
        
        // Try primary engine first
        if let Some(engine) = self.engines.get(&primary_engine) {
            match engine.execute_query(query.clone()).await {
                Ok(mut result) => {
                    result.engine_name = primary_engine;
                    return Ok(result);
                }
                Err(e) if !e.is_retryable() => return Err(e),
                _ => {} // Continue to fallback
            }
        }
        
        // Try fallback engines
        for engine_name in &self.fallback_chain {
            if engine_name == &primary_engine {
                continue; // Already tried
            }
            
            if let Some(engine) = self.engines.get(engine_name) {
                if engine.can_handle_query(&query) {
                    match engine.execute_query(query.clone()).await {
                        Ok(mut result) => {
                            result.engine_name = engine_name.clone();
                            return Ok(result);
                        }
                        Err(e) if !e.is_retryable() => return Err(e),
                        _ => continue,
                    }
                }
            }
        }
        
        Err(IntegrationError::InternalError {
            message: "No available engine could handle the query".to_string(),
        })
    }
    
    /// Get all available engines and their capabilities
    pub fn get_engine_capabilities(&self) -> HashMap<String, Vec<ComputationalCapability>> {
        self.engines
            .iter()
            .map(|(name, engine)| (name.clone(), engine.supported_capabilities()))
            .collect()
    }
    
    /// Get health status of all engines
    pub async fn get_engines_health(&self) -> HashMap<String, IntegrationResult<IntegrationHealth>> {
        let mut results = HashMap::new();
        
        for (name, engine) in &self.engines {
            let health = engine.health_check().await;
            results.insert(name.clone(), health);
        }
        
        results
    }
    
    /// Select the best engine for a query based on the routing strategy
    fn select_engine(&self, query: &ComputationalQuery) -> IntegrationResult<String> {
        let capable_engines: Vec<_> = self.engines
            .iter()
            .filter(|(_, engine)| engine.can_handle_query(query))
            .collect();
        
        if capable_engines.is_empty() {
            return Err(IntegrationError::FeatureNotSupported {
                feature: format!("Query with capabilities: {:?}", query.capabilities_required),
            });
        }
        
        match self.routing_strategy {
            RoutingStrategy::FirstCapable => {
                Ok(capable_engines[0].0.clone())
            }
            RoutingStrategy::BestMatch => {
                // Select engine with most matching capabilities
                let best = capable_engines
                    .iter()
                    .max_by_key(|(_, engine)| {
                        let engine_caps = engine.supported_capabilities();
                        query.capabilities_required
                            .iter()
                            .filter(|cap| engine_caps.contains(cap))
                            .count()
                    });
                Ok(best.unwrap().0.clone())
            }
            RoutingStrategy::Fastest => {
                // For now, just return first capable
                // In a real implementation, you'd track performance metrics
                Ok(capable_engines[0].0.clone())
            }
            RoutingStrategy::CostEffective => {
                // For now, just return first capable
                // In a real implementation, you'd consider cost metrics
                Ok(capable_engines[0].0.clone())
            }
            RoutingStrategy::Custom => {
                // Implement custom logic here
                Ok(capable_engines[0].0.clone())
            }
        }
    }
}

/// Helper functions for creating common query types
impl ComputationalQuery {
    /// Create a natural language query
    pub fn natural_language(query: &str) -> Self {
        Self {
            query_id: QueryId::new(),
            input: QueryInputFormat::NaturalLanguage(query.to_string()),
            capabilities_required: vec![ComputationalCapability::NaturalLanguageQuery],
            output_format: OutputFormat::PlainText,
            priority: QueryPriority::Normal,
            timeout: Some(Duration::from_secs(30)),
            context: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a mathematical expression query
    pub fn mathematical_expression(expression: &str, notation: MathNotation) -> Self {
        Self {
            query_id: QueryId::new(),
            input: QueryInputFormat::Mathematical {
                expression: expression.to_string(),
                notation,
            },
            capabilities_required: vec![ComputationalCapability::BasicMath],
            output_format: OutputFormat::LaTeX,
            priority: QueryPriority::Normal,
            timeout: Some(Duration::from_secs(30)),
            context: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a structured query
    pub fn structured(operation: &str, parameters: HashMap<String, serde_json::Value>) -> Self {
        Self {
            query_id: QueryId::new(),
            input: QueryInputFormat::Structured {
                operation: operation.to_string(),
                parameters,
            },
            capabilities_required: vec![ComputationalCapability::BasicMath],
            output_format: OutputFormat::JSON,
            priority: QueryPriority::Normal,
            timeout: Some(Duration::from_secs(30)),
            context: None,
            metadata: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_id_generation() {
        let id1 = QueryId::new();
        let id2 = QueryId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_natural_language_query_creation() {
        let query = ComputationalQuery::natural_language("integrate x^2 from 0 to 5");
        assert!(matches!(query.input, QueryInputFormat::NaturalLanguage(_)));
        assert_eq!(query.priority, QueryPriority::Normal);
    }

    #[test]
    fn test_mathematical_expression_query_creation() {
        let query = ComputationalQuery::mathematical_expression("x^2 + 2x + 1", MathNotation::LaTeX);
        assert!(matches!(query.input, QueryInputFormat::Mathematical { .. }));
        assert!(matches!(query.output_format, OutputFormat::LaTeX));
    }
}
