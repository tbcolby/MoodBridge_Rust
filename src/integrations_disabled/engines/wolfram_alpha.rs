//! # Wolfram Alpha Computational Engine Plugin
//! 
//! This module implements the Wolfram Alpha integration as a computational engine plugin.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use reqwest::Client;

use crate::integrations::{
    IntegrationResult, IntegrationError, IntegrationHealth, ConnectionStatus, 
    PlatformIntegration, IntegrationConfig, IntegrationCapability,
    AuthenticationResult,
};

use super::super::computational::{
    ComputationalEngine, ComputationalCapability, ComputationalQuery, ComputationalResult,
    QueryId, QueryStatus, EngineUsageStats, ValidationResult, QueryInputFormat,
    OutputFormat, QueryOutput, QueryCost, VisualizationData, RateLimitStatus, MathNotation,
};

/// Wolfram Alpha API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolframAlphaConfig {
    pub app_id: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub rate_limit_per_hour: Option<u32>,
    pub enable_step_by_step: bool,
    pub enable_plots: bool,
    pub preferred_units: Option<String>,
}

impl Default for WolframAlphaConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            base_url: "https://api.wolframalpha.com/v2/query".to_string(),
            timeout_seconds: 30,
            rate_limit_per_hour: Some(2000),
            enable_step_by_step: true,
            enable_plots: true,
            preferred_units: Some("metric".to_string()),
        }
    }
}

/// Wolfram Alpha API response structures
#[derive(Debug, Deserialize)]
struct WolframAlphaResponse {
    #[serde(rename = "queryresult")]
    query_result: QueryResult,
}

#[derive(Debug, Deserialize)]
struct QueryResult {
    success: bool,
    error: bool,
    numpods: u32,
    datatypes: Option<String>,
    timedout: Option<String>,
    timedoutpods: Option<String>,
    timing: Option<f64>,
    parsetiming: Option<f64>,
    parsetimedout: Option<bool>,
    recalculate: Option<String>,
    id: Option<String>,
    host: Option<String>,
    server: Option<String>,
    related: Option<String>,
    version: Option<String>,
    pods: Option<Vec<Pod>>,
    assumptions: Option<Vec<Assumption>>,
    sources: Option<Vec<Source>>,
    error_details: Option<ErrorDetails>,
}

#[derive(Debug, Deserialize)]
struct Pod {
    title: String,
    scanner: Option<String>,
    id: Option<String>,
    position: Option<u32>,
    error: Option<bool>,
    numsubpods: Option<u32>,
    subpods: Option<Vec<SubPod>>,
    expressiontypes: Option<String>,
    states: Option<Vec<State>>,
}

#[derive(Debug, Deserialize)]
struct SubPod {
    title: Option<String>,
    img: Option<Image>,
    plaintext: Option<String>,
    mathml: Option<String>,
    latex: Option<String>,
    microsyntax: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Image {
    src: String,
    alt: String,
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    #[serde(rename = "type")]
    image_type: Option<String>,
    themes: Option<String>,
    colorinvertible: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct State {
    name: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct Assumption {
    #[serde(rename = "type")]
    assumption_type: String,
    word: Option<String>,
    template: Option<String>,
    count: Option<u32>,
    values: Option<Vec<AssumptionValue>>,
}

#[derive(Debug, Deserialize)]
struct AssumptionValue {
    name: String,
    desc: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct Source {
    url: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ErrorDetails {
    code: Option<String>,
    msg: Option<String>,
}

/// Wolfram Alpha computational engine implementation
pub struct WolframAlphaEngine {
    config: WolframAlphaConfig,
    client: Client,
    usage_stats: EngineUsageStats,
    last_request_time: Option<Instant>,
}

impl WolframAlphaEngine {
    pub fn new(config: WolframAlphaConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            client,
            usage_stats: EngineUsageStats {
                total_queries: 0,
                successful_queries: 0,
                failed_queries: 0,
                average_execution_time_ms: 0.0,
                total_cost: None,
                rate_limit_status: RateLimitStatus {
                    requests_remaining: None,
                    reset_time: None,
                    daily_limit: None,
                    monthly_limit: None,
                },
                last_query_time: None,
            },
            last_request_time: None,
        }
    }

    /// Build the Wolfram Alpha API URL with parameters
    fn build_api_url(&self, query: &str, output_format: &str) -> String {
        let mut url = format!("{}?appid={}&input={}", 
            self.config.base_url, 
            self.config.app_id, 
            urlencoding::encode(query)
        );
        
        url.push_str(&format!("&format={}", output_format));
        
        if self.config.enable_step_by_step {
            url.push_str("&podstate=Step-by-step+solution");
        }
        
        if self.config.enable_plots {
            url.push_str("&includepodid=Plot");
        }
        
        if let Some(units) = &self.config.preferred_units {
            url.push_str(&format!("&units={}", units));
        }
        
        url.push_str("&output=json");
        url
    }

    /// Convert Wolfram Alpha response to our result format
    fn convert_response(&self, response: WolframAlphaResponse, query_id: QueryId, start_time: Instant) -> ComputationalResult {
        let execution_time = start_time.elapsed().as_millis() as u64;
        let query_result = response.query_result;
        
        if !query_result.success || query_result.error {
            return ComputationalResult {
                query_id,
                engine_name: "wolfram_alpha".to_string(),
                success: false,
                result: None,
                error: query_result.error_details
                    .map(|e| e.msg.unwrap_or_else(|| "Unknown error".to_string()))
                    .or_else(|| Some("Query failed".to_string())),
                execution_time_ms: execution_time,
                cost: Some(QueryCost {
                    credits_used: Some(1),
                    monetary_cost: None,
                    currency: None,
                    rate_limit_consumed: Some(1),
                }),
                confidence: None,
                alternatives: Vec::new(),
                metadata: HashMap::new(),
            };
        }

        let mut outputs = Vec::new();
        
        if let Some(pods) = query_result.pods {
            for pod in pods {
                if let Some(subpods) = pod.subpods {
                    for subpod in subpods {
                        // Create output for each subpod
                        if let Some(plaintext) = subpod.plaintext {
                            if !plaintext.is_empty() {
                                outputs.push(QueryOutput {
                                    format: OutputFormat::PlainText,
                                    content: serde_json::Value::String(plaintext),
                                    description: Some(pod.title.clone()),
                                    visualization: subpod.img.map(|img| VisualizationData {
                                        type_: "image".to_string(),
                                        data: serde_json::json!({
                                            "src": img.src,
                                            "alt": img.alt,
                                            "width": img.width,
                                            "height": img.height
                                        }),
                                        settings: HashMap::new(),
                                    }),
                                    references: Vec::new(),
                                });
                            }
                        }
                        
                        if let Some(latex) = subpod.latex {
                            if !latex.is_empty() {
                                outputs.push(QueryOutput {
                                    format: OutputFormat::LaTeX,
                                    content: serde_json::Value::String(latex),
                                    description: Some(format!("{} (LaTeX)", pod.title)),
                                    visualization: None,
                                    references: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }

        let primary_result = outputs.first().cloned();
        let alternatives = if outputs.len() > 1 {
            outputs[1..].to_vec()
        } else {
            Vec::new()
        };

        // Add sources as references
        let references = query_result.sources
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.url)
            .collect();

        if let Some(mut result) = primary_result {
            result.references = references;
        }

        ComputationalResult {
            query_id,
            engine_name: "wolfram_alpha".to_string(),
            success: true,
            result: primary_result,
            error: None,
            execution_time_ms: execution_time,
            cost: Some(QueryCost {
                credits_used: Some(1),
                monetary_cost: None,
                currency: None,
                rate_limit_consumed: Some(1),
            }),
            confidence: Some(0.9), // Wolfram Alpha is generally high confidence
            alternatives,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("timing".to_string(), 
                    serde_json::json!(query_result.timing.unwrap_or(0.0)));
                meta.insert("numpods".to_string(), 
                    serde_json::Value::Number(query_result.numpods.into()));
                meta
            },
        }
    }

    /// Update usage statistics
    fn update_stats(&mut self, success: bool, execution_time_ms: u64) {
        self.usage_stats.total_queries += 1;
        if success {
            self.usage_stats.successful_queries += 1;
        } else {
            self.usage_stats.failed_queries += 1;
        }
        
        let total_time = self.usage_stats.average_execution_time_ms * (self.usage_stats.total_queries - 1) as f64;
        self.usage_stats.average_execution_time_ms = (total_time + execution_time_ms as f64) / self.usage_stats.total_queries as f64;
        
        self.usage_stats.last_query_time = Some(Utc::now());
    }
}

#[async_trait]
impl PlatformIntegration for WolframAlphaEngine {
    fn platform_name(&self) -> &'static str {
        "wolfram_alpha"
    }

    fn capabilities(&self) -> Vec<IntegrationCapability> {
        vec![
            IntegrationCapability::Analytics,
        ]
    }

    async fn health_check(&self) -> IntegrationResult<IntegrationHealth> {
        // Simple health check by making a basic query
        let test_query = "2+2";
        let url = self.build_api_url(test_query, "plaintext");
        
        let start = Instant::now();
        let response = self.client.get(&url).send().await;
        let response_time = start.elapsed().as_millis() as u64;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                Ok(IntegrationHealth {
                    platform_name: "Wolfram Alpha".to_string(),
                    status: ConnectionStatus::Healthy,
                    last_checked: Utc::now(),
                    response_time_ms: Some(response_time),
                    capabilities: vec![IntegrationCapability::Analytics],
                    rate_limit_remaining: self.config.rate_limit_per_hour,
                    rate_limit_reset: None,
                    metadata: HashMap::new(),
                })
            }
            Ok(resp) => {
                Ok(IntegrationHealth {
                    platform_name: "Wolfram Alpha".to_string(),
                    status: ConnectionStatus::Degraded { 
                        reason: format!("HTTP {}", resp.status()) 
                    },
                    last_checked: Utc::now(),
                    response_time_ms: Some(response_time),
                    capabilities: vec![IntegrationCapability::Analytics],
                    rate_limit_remaining: self.config.rate_limit_per_hour,
                    rate_limit_reset: None,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                Ok(IntegrationHealth {
                    platform_name: "Wolfram Alpha".to_string(),
                    status: ConnectionStatus::Unhealthy { 
                        error: e.to_string() 
                    },
                    last_checked: Utc::now(),
                    response_time_ms: Some(response_time),
                    capabilities: vec![IntegrationCapability::Analytics],
                    rate_limit_remaining: self.config.rate_limit_per_hour,
                    rate_limit_reset: None,
                    metadata: HashMap::new(),
                })
            }
        }
    }

    async fn initialize(&mut self, config: &IntegrationConfig) -> IntegrationResult<()> {
        // Extract Wolfram Alpha specific configuration
        if let Some(app_id) = config.custom_settings.get("app_id") {
            if let Some(app_id_str) = app_id.as_str() {
                self.config.app_id = app_id_str.to_string();
            }
        }
        
        if self.config.app_id.is_empty() {
            return Err(IntegrationError::ConfigurationError {
                message: "Wolfram Alpha App ID is required".to_string(),
            });
        }
        
        Ok(())
    }

    async fn shutdown(&mut self) -> IntegrationResult<()> {
        // Nothing specific to cleanup for Wolfram Alpha
        Ok(())
    }

    async fn authenticate(&mut self) -> IntegrationResult<AuthenticationResult> {
        // Wolfram Alpha uses App ID, which is set during initialization
        Ok(AuthenticationResult {
            success: !self.config.app_id.is_empty(),
            access_token: None,
            refresh_token: None,
            expires_at: None,
            token_type: Some("AppID".to_string()),
            scope: None,
            metadata: HashMap::new(),
        })
    }

    async fn refresh_auth(&mut self) -> IntegrationResult<AuthenticationResult> {
        // App ID doesn't need refreshing
        self.authenticate().await
    }
}

#[async_trait]
impl ComputationalEngine for WolframAlphaEngine {
    fn supported_capabilities(&self) -> Vec<ComputationalCapability> {
        vec![
            ComputationalCapability::BasicMath,
            ComputationalCapability::AdvancedMath,
            ComputationalCapability::Statistics,
            ComputationalCapability::Physics,
            ComputationalCapability::Chemistry,
            ComputationalCapability::Engineering,
            ComputationalCapability::NaturalLanguageQuery,
            ComputationalCapability::SymbolicMath,
            ComputationalCapability::NumericalAnalysis,
            ComputationalCapability::FinancialMath,
            ComputationalCapability::UnitConversion,
        ]
    }

    fn supported_input_formats(&self) -> Vec<QueryInputFormat> {
        vec![
            QueryInputFormat::NaturalLanguage("example".to_string()),
            QueryInputFormat::Mathematical {
                expression: "example".to_string(),
                notation: MathNotation::Standard,
            },
            QueryInputFormat::Mathematical {
                expression: "example".to_string(),
                notation: MathNotation::WolframLanguage,
            },
        ]
    }

    fn supported_output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::PlainText,
            OutputFormat::LaTeX,
            OutputFormat::Image { format: "PNG".to_string() },
            OutputFormat::HTML,
        ]
    }

    fn can_handle_query(&self, query: &ComputationalQuery) -> bool {
        let supported_caps = self.supported_capabilities();
        
        // Check if we support at least one required capability
        query.capabilities_required.iter().any(|cap| supported_caps.contains(cap)) &&
        
        // Check if input format is supported
        match &query.input {
            QueryInputFormat::NaturalLanguage(_) => true,
            QueryInputFormat::Mathematical { notation, .. } => {
                matches!(notation, MathNotation::Standard | MathNotation::WolframLanguage)
            }
            _ => false,
        }
    }

    async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        let start_time = Instant::now();
        let query_id = query.query_id.clone();
        
        // Extract the query string based on input format
        let query_string = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text.clone(),
            QueryInputFormat::Mathematical { expression, .. } => expression.clone(),
            QueryInputFormat::Structured { operation, parameters } => {
                // Convert structured query to Wolfram Alpha format
                format!("{} {}", operation, 
                    parameters.values()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
            }
            QueryInputFormat::Code { code, .. } => code.clone(),
        };
        
        // Determine output format for API
        let api_format = match query.output_format {
            OutputFormat::PlainText => "plaintext",
            OutputFormat::LaTeX => "mathml,latex",
            OutputFormat::Image { .. } => "image",
            _ => "plaintext",
        };
        
        let url = self.build_api_url(&query_string, api_format);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| IntegrationError::NetworkError(e))?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::ApiError {
                status_code: response.status().as_u16(),
                message: "Wolfram Alpha API request failed".to_string(),
            });
        }
        
        let wolfram_response: WolframAlphaResponse = response.json().await
            .map_err(|e| IntegrationError::InternalError {
                message: format!("Failed to parse response: {}", e)
            })?;
        
        Ok(self.convert_response(wolfram_response, query_id, start_time))
    }

    async fn get_query_status(&self, _query_id: &QueryId) -> IntegrationResult<QueryStatus> {
        // Wolfram Alpha doesn't support async queries - all queries are synchronous
        Ok(QueryStatus::Completed)
    }

    async fn cancel_query(&self, _query_id: &QueryId) -> IntegrationResult<()> {
        // Wolfram Alpha doesn't support query cancellation
        Err(IntegrationError::FeatureNotSupported {
            feature: "Query cancellation".to_string(),
        })
    }

    async fn get_usage_stats(&self) -> IntegrationResult<EngineUsageStats> {
        Ok(self.usage_stats.clone())
    }

    async fn validate_query(&self, query: &ComputationalQuery) -> IntegrationResult<ValidationResult> {
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check if query is too complex or long
        let query_text = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text,
            QueryInputFormat::Mathematical { expression, .. } => expression,
            _ => "",
        };
        
        if query_text.len() > 1000 {
            warnings.push("Query is very long and may timeout".to_string());
        }
        
        if query_text.contains("plot") || query_text.contains("graph") {
            if !self.config.enable_plots {
                suggestions.push("Enable plots in configuration for better results".to_string());
            }
        }
        
        let estimated_cost = Some(QueryCost {
            credits_used: Some(1),
            monetary_cost: None,
            currency: None,
            rate_limit_consumed: Some(1),
        });
        
        let estimated_time = if query_text.len() > 100 {
            Some(Duration::from_secs(10))
        } else {
            Some(Duration::from_secs(5))
        };
        
        Ok(ValidationResult {
            is_valid: self.can_handle_query(query),
            estimated_cost,
            estimated_execution_time: estimated_time,
            warnings,
            suggestions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integrations::computational::*;

    #[test]
    fn test_wolfram_alpha_engine_creation() {
        let config = WolframAlphaConfig::default();
        let engine = WolframAlphaEngine::new(config);
        assert_eq!(engine.platform_name(), "wolfram_alpha");
    }

    #[test]
    fn test_supported_capabilities() {
        let config = WolframAlphaConfig::default();
        let engine = WolframAlphaEngine::new(config);
        let capabilities = engine.supported_capabilities();
        
        assert!(capabilities.contains(&ComputationalCapability::BasicMath));
        assert!(capabilities.contains(&ComputationalCapability::NaturalLanguageQuery));
        assert!(capabilities.contains(&ComputationalCapability::AdvancedMath));
    }

    #[test]
    fn test_can_handle_query() {
        let config = WolframAlphaConfig::default();
        let engine = WolframAlphaEngine::new(config);
        
        let query = ComputationalQuery::natural_language("integrate x^2");
        assert!(engine.can_handle_query(&query));
        
        let math_query = ComputationalQuery::mathematical_expression(
            "x^2 + 1", 
            MathNotation::Standard
        );
        assert!(engine.can_handle_query(&math_query));
    }

    #[test]
    fn test_api_url_building() {
        let mut config = WolframAlphaConfig::default();
        config.app_id = "TEST123".to_string();
        let engine = WolframAlphaEngine::new(config);
        
        let url = engine.build_api_url("2+2", "plaintext");
        assert!(url.contains("appid=TEST123"));
        assert!(url.contains("input=2%2B2"));
        assert!(url.contains("format=plaintext"));
    }
}
