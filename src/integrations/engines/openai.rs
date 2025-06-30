//! # OpenAI/ChatGPT Computational Engine Plugin
//! 
//! This module implements the OpenAI integration as a computational engine plugin.
//! It leverages ChatGPT's natural language understanding for mathematical problems,
//! code generation, explanations, and complex reasoning tasks.

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

/// OpenAI engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_seconds: u64,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub enable_code_generation: bool,
    pub enable_explanations: bool,
    pub enable_step_by_step: bool,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            timeout_seconds: 60,
            max_tokens: Some(2048),
            temperature: Some(0.1), // Low temperature for consistent math results
            enable_code_generation: true,
            enable_explanations: true,
            enable_step_by_step: true,
        }
    }
}

/// OpenAI API request structure
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI API response structure
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    index: u32,
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// OpenAI computational engine implementation
pub struct OpenAIEngine {
    config: OpenAIConfig,
    client: Client,
    usage_stats: EngineUsageStats,
}

impl OpenAIEngine {
    pub fn new(config: OpenAIConfig) -> Self {
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
                total_cost: Some(0.0),
                rate_limit_status: RateLimitStatus {
                    requests_remaining: None,
                    reset_time: None,
                    daily_limit: None,
                    monthly_limit: None,
                },
                last_query_time: None,
            },
        }
    }

    /// Create system prompt for mathematical queries
    fn create_system_prompt(&self, query: &ComputationalQuery) -> String {
        let mut prompt = String::from(
            "You are an expert mathematician and computational assistant. Your role is to solve mathematical problems accurately and provide clear explanations."
        );

        if self.config.enable_step_by_step {
            prompt.push_str(" Always show your work step-by-step.");
        }

        match query.output_format {
            OutputFormat::LaTeX => {
                prompt.push_str(" Format mathematical expressions using LaTeX notation.");
            }
            OutputFormat::JSON => {
                prompt.push_str(" Provide your response in JSON format with fields: 'answer', 'explanation', 'steps', and 'confidence'.");
            }
            OutputFormat::Markdown => {
                prompt.push_str(" Format your response in clean Markdown with proper mathematical notation.");
            }
            _ => {}
        }

        if self.config.enable_code_generation && query.capabilities_required.contains(&ComputationalCapability::MachineLearning) {
            prompt.push_str(" When appropriate, provide Python code examples using libraries like NumPy, SciPy, or SymPy.");
        }

        prompt
    }

    /// Create user prompt from query
    fn create_user_prompt(&self, query: &ComputationalQuery) -> String {
        let base_query = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text.clone(),
            QueryInputFormat::Mathematical { expression, notation } => {
                match notation {
                    MathNotation::LaTeX => format!("Solve this mathematical expression: ${expression}$"),
                    _ => format!("Solve this mathematical expression: {expression}"),
                }
            }
            QueryInputFormat::Structured { operation, parameters } => {
                format!("Perform the operation '{}' with parameters: {}", 
                    operation,
                    parameters.iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            QueryInputFormat::Code { language, code } => {
                format!("Analyze and explain this {} code, and provide the mathematical result:\n```{}\n{}\n```", 
                    language, language, code)
            }
        };

        let mut prompt = base_query;

        // Add context based on required capabilities
        if query.capabilities_required.contains(&ComputationalCapability::Statistics) {
            prompt.push_str("\nProvide statistical analysis including relevant measures and interpretations.");
        }

        if query.capabilities_required.contains(&ComputationalCapability::Physics) {
            prompt.push_str("\nInclude physical interpretations and units where applicable.");
        }

        if query.capabilities_required.contains(&ComputationalCapability::FinancialMath) {
            prompt.push_str("\nProvide financial context and practical applications.");
        }

        // Add domain context if provided
        if let Some(context) = &query.context {
            if let Some(domain) = &context.domain {
                prompt.push_str(&format!("\nContext: This problem is related to {}.", domain));
            }
        }

        prompt
    }

    /// Estimate cost based on tokens
    fn estimate_cost(&self, prompt_tokens: u32, completion_tokens: u32) -> QueryCost {
        // GPT-4 pricing (as of 2024): $0.03 per 1K prompt tokens, $0.06 per 1K completion tokens
        let prompt_cost = (prompt_tokens as f64 / 1000.0) * 0.03;
        let completion_cost = (completion_tokens as f64 / 1000.0) * 0.06;
        let total_cost = prompt_cost + completion_cost;

        QueryCost {
            credits_used: Some(prompt_tokens + completion_tokens),
            monetary_cost: Some(total_cost),
            currency: Some("USD".to_string()),
            rate_limit_consumed: Some(1),
        }
    }

    /// Parse OpenAI response and extract mathematical content
    fn parse_mathematical_content(&self, content: &str, output_format: &OutputFormat) -> Vec<QueryOutput> {
        let mut outputs = Vec::new();

        match output_format {
            OutputFormat::JSON => {
                // Try to parse as JSON first
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(content) {
                    outputs.push(QueryOutput {
                        format: OutputFormat::JSON,
                        content: json_value,
                        description: Some("Structured mathematical result".to_string()),
                        visualization: None,
                        references: Vec::new(),
                    });
                } else {
                    // Fallback to plain text
                    outputs.push(QueryOutput {
                        format: OutputFormat::PlainText,
                        content: serde_json::Value::String(content.to_string()),
                        description: Some("Mathematical explanation".to_string()),
                        visualization: None,
                        references: Vec::new(),
                    });
                }
            }
            OutputFormat::LaTeX => {
                // Extract LaTeX expressions
                let latex_pattern = regex::Regex::new(r"\$([^$]+)\$").unwrap();
                let latex_matches: Vec<_> = latex_pattern.captures_iter(content)
                    .map(|cap| cap[1].to_string())
                    .collect();

                if !latex_matches.is_empty() {
                    for latex_expr in latex_matches {
                        outputs.push(QueryOutput {
                            format: OutputFormat::LaTeX,
                            content: serde_json::Value::String(latex_expr),
                            description: Some("LaTeX mathematical expression".to_string()),
                            visualization: None,
                            references: Vec::new(),
                        });
                    }
                }

                // Also include full text
                outputs.push(QueryOutput {
                    format: OutputFormat::PlainText,
                    content: serde_json::Value::String(content.to_string()),
                    description: Some("Complete explanation".to_string()),
                    visualization: None,
                    references: Vec::new(),
                });
            }
            OutputFormat::Markdown => {
                outputs.push(QueryOutput {
                    format: OutputFormat::Markdown,
                    content: serde_json::Value::String(content.to_string()),
                    description: Some("Markdown formatted result".to_string()),
                    visualization: None,
                    references: Vec::new(),
                });
            }
            _ => {
                outputs.push(QueryOutput {
                    format: OutputFormat::PlainText,
                    content: serde_json::Value::String(content.to_string()),
                    description: Some("Mathematical result and explanation".to_string()),
                    visualization: None,
                    references: Vec::new(),
                });
            }
        }

        // Extract code blocks if code generation is enabled
        if self.config.enable_code_generation {
            let code_pattern = regex::Regex::new(r"```(\w+)\n([^`]+)\n```").unwrap();
            for cap in code_pattern.captures_iter(content) {
                let language = &cap[1];
                let code = &cap[2];
                
                outputs.push(QueryOutput {
                    format: OutputFormat::EngineSpecific { format: "code".to_string() },
                    content: serde_json::json!({
                        "language": language,
                        "code": code
                    }),
                    description: Some(format!("{} code solution", language)),
                    visualization: None,
                    references: Vec::new(),
                });
            }
        }

        outputs
    }

    /// Update usage statistics
    fn update_stats(&mut self, success: bool, execution_time_ms: u64, cost: Option<f64>) {
        self.usage_stats.total_queries += 1;
        if success {
            self.usage_stats.successful_queries += 1;
        } else {
            self.usage_stats.failed_queries += 1;
        }
        
        let total_time = self.usage_stats.average_execution_time_ms * (self.usage_stats.total_queries - 1) as f64;
        self.usage_stats.average_execution_time_ms = (total_time + execution_time_ms as f64) / self.usage_stats.total_queries as f64;
        
        if let Some(cost) = cost {
            let current_total = self.usage_stats.total_cost.unwrap_or(0.0);
            self.usage_stats.total_cost = Some(current_total + cost);
        }
        
        self.usage_stats.last_query_time = Some(Utc::now());
    }
}

#[async_trait]
impl PlatformIntegration for OpenAIEngine {
    fn platform_name(&self) -> &'static str {
        "openai"
    }

    fn capabilities(&self) -> Vec<IntegrationCapability> {
        vec![
            IntegrationCapability::Analytics,
        ]
    }

    async fn health_check(&self) -> IntegrationResult<IntegrationHealth> {
        let start = Instant::now();
        
        // Test with a simple math query
        let test_request = OpenAIRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a math assistant. Respond with just the number.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: "What is 2 + 2?".to_string(),
                },
            ],
            max_tokens: Some(10),
            temperature: Some(0.0),
            stream: false,
        };

        let status = match self.client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&test_request)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => ConnectionStatus::Healthy,
            Ok(response) => ConnectionStatus::Degraded { 
                reason: format!("HTTP {}", response.status()) 
            },
            Err(e) => ConnectionStatus::Unhealthy { 
                error: e.to_string() 
            },
        };

        let response_time = start.elapsed().as_millis() as u64;

        Ok(IntegrationHealth {
            platform_name: "OpenAI".to_string(),
            status,
            last_checked: Utc::now(),
            response_time_ms: Some(response_time),
            capabilities: vec![IntegrationCapability::Analytics],
            rate_limit_remaining: None, // Would need to parse headers for actual limits
            rate_limit_reset: None,
            metadata: HashMap::new(),
        })
    }

    async fn initialize(&mut self, config: &IntegrationConfig) -> IntegrationResult<()> {
        if let Some(api_key) = config.custom_settings.get("api_key") {
            if let Some(api_key_str) = api_key.as_str() {
                self.config.api_key = api_key_str.to_string();
            }
        }
        
        if self.config.api_key.is_empty() {
            return Err(IntegrationError::ConfigurationError {
                message: "OpenAI API key is required".to_string(),
            });
        }
        
        Ok(())
    }

    async fn shutdown(&mut self) -> IntegrationResult<()> {
        Ok(())
    }

    async fn authenticate(&mut self) -> IntegrationResult<AuthenticationResult> {
        Ok(AuthenticationResult {
            success: !self.config.api_key.is_empty(),
            access_token: None,
            refresh_token: None,
            expires_at: None,
            token_type: Some("Bearer".to_string()),
            scope: None,
            metadata: HashMap::new(),
        })
    }

    async fn refresh_auth(&mut self) -> IntegrationResult<AuthenticationResult> {
        self.authenticate().await
    }
}

#[async_trait]
impl ComputationalEngine for OpenAIEngine {
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
            ComputationalCapability::MachineLearning,
            ComputationalCapability::DataAnalysis,
            ComputationalCapability::GraphTheory,
            ComputationalCapability::SignalProcessing,
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
                notation: MathNotation::LaTeX,
            },
            QueryInputFormat::Structured {
                operation: "example".to_string(),
                parameters: HashMap::new(),
            },
            QueryInputFormat::Code {
                language: "python".to_string(),
                code: "example".to_string(),
            },
        ]
    }

    fn supported_output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::PlainText,
            OutputFormat::LaTeX,
            OutputFormat::JSON,
            OutputFormat::Markdown,
            OutputFormat::EngineSpecific { format: "code".to_string() },
        ]
    }

    fn can_handle_query(&self, query: &ComputationalQuery) -> bool {
        if self.config.api_key.is_empty() {
            return false;
        }

        let supported_caps = self.supported_capabilities();
        
        // OpenAI can handle almost any query due to its natural language understanding
        query.capabilities_required.iter().any(|cap| supported_caps.contains(cap)) ||
        query.capabilities_required.is_empty() // Default to handling general queries
    }

    async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        let start_time = Instant::now();
        let query_id = query.query_id.clone();
        
        let system_prompt = self.create_system_prompt(&query);
        let user_prompt = self.create_user_prompt(&query);
        
        let request = OpenAIRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            stream: false,
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| IntegrationError::NetworkError(e))?;

        if !response.status().is_success() {
            return Err(IntegrationError::ApiError {
                status_code: response.status().as_u16(),
                message: "OpenAI API request failed".to_string(),
            });
        }

        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| IntegrationError::InternalError {
                message: format!("Failed to parse OpenAI response: {}", e),
            })?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        if openai_response.choices.is_empty() {
            return Err(IntegrationError::InternalError {
                message: "No response from OpenAI".to_string(),
            });
        }

        let choice = &openai_response.choices[0];
        let content = &choice.message.content;
        
        let outputs = self.parse_mathematical_content(content, &query.output_format);
        let primary_result = outputs.first().cloned();
        let alternatives = if outputs.len() > 1 {
            outputs[1..].to_vec()
        } else {
            Vec::new()
        };

        // Calculate cost
        let cost = if let Some(usage) = &openai_response.usage {
            Some(self.estimate_cost(usage.prompt_tokens, usage.completion_tokens))
        } else {
            None
        };

        Ok(ComputationalResult {
            query_id,
            engine_name: "openai".to_string(),
            success: true,
            result: primary_result,
            error: None,
            execution_time_ms: execution_time,
            cost,
            confidence: Some(0.85), // High confidence due to GPT-4's capabilities
            alternatives,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("model".to_string(), serde_json::Value::String(openai_response.model));
                if let Some(usage) = &openai_response.usage {
                    meta.insert("tokens_used".to_string(), serde_json::Value::Number(usage.total_tokens.into()));
                }
                meta
            },
        })
    }

    async fn get_query_status(&self, _query_id: &QueryId) -> IntegrationResult<QueryStatus> {
        // OpenAI API is synchronous
        Ok(QueryStatus::Completed)
    }

    async fn cancel_query(&self, _query_id: &QueryId) -> IntegrationResult<()> {
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
        
        let query_text = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text,
            QueryInputFormat::Mathematical { expression, .. } => expression,
            _ => "",
        };
        
        // Estimate token usage
        let estimated_tokens = query_text.len() / 4; // Rough estimate: 1 token ≈ 4 characters
        
        if estimated_tokens > 3000 {
            warnings.push("Large query may result in high token usage and cost".to_string());
        }
        
        if query.capabilities_required.contains(&ComputationalCapability::SymbolicMath) {
            suggestions.push("For complex symbolic math, consider using SymPy engine for more precise results".to_string());
        }
        
        // Estimate cost
        let estimated_cost = if estimated_tokens > 0 {
            Some(self.estimate_cost(estimated_tokens as u32, 500)) // Assume 500 completion tokens
        } else {
            None
        };

        Ok(ValidationResult {
            is_valid: self.can_handle_query(query),
            estimated_cost,
            estimated_execution_time: Some(Duration::from_secs(10)),
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
    fn test_openai_engine_creation() {
        let config = OpenAIConfig::default();
        let engine = OpenAIEngine::new(config);
        assert_eq!(engine.platform_name(), "openai");
    }

    #[test]
    fn test_supported_capabilities() {
        let config = OpenAIConfig::default();
        let engine = OpenAIEngine::new(config);
        let capabilities = engine.supported_capabilities();
        
        assert!(capabilities.contains(&ComputationalCapability::NaturalLanguageQuery));
        assert!(capabilities.contains(&ComputationalCapability::MachineLearning));
        assert!(capabilities.contains(&ComputationalCapability::BasicMath));
    }

    #[test]
    fn test_prompt_generation() {
        let config = OpenAIConfig::default();
        let engine = OpenAIEngine::new(config);
        
        let query = ComputationalQuery::natural_language("solve x^2 + 2x + 1 = 0");
        let system_prompt = engine.create_system_prompt(&query);
        let user_prompt = engine.create_user_prompt(&query);
        
        assert!(system_prompt.contains("mathematician"));
        assert!(user_prompt.contains("solve x^2 + 2x + 1 = 0"));
    }

    #[test]
    fn test_cost_estimation() {
        let config = OpenAIConfig::default();
        let engine = OpenAIEngine::new(config);
        
        let cost = engine.estimate_cost(1000, 500);
        assert!(cost.monetary_cost.unwrap() > 0.0);
        assert_eq!(cost.currency.as_deref(), Some("USD"));
    }
}
