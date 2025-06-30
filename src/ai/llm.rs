use crate::ai::{AiConfig, AiError, AiService, AnalysisRequest, AnalysisResponse, AiInsight, InsightType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::Utc;

/// OpenAI API service implementation
pub struct OpenAiService {
    client: Client,
    config: AiConfig,
}

impl OpenAiService {
    pub fn new(config: AiConfig) -> Self {
        let client = Client::new();
        Self { client, config }
    }

    /// Create a completion request to OpenAI API
    async fn create_completion(&self, messages: Vec<OpenAiMessage>) -> Result<String, AiError> {
        let api_key = self.config.openai_api_key
            .as_ref()
            .ok_or_else(|| AiError::ConfigError("OpenAI API key not configured".to_string()))?;

        let request = OpenAiRequest {
            model: self.config.default_model.clone(),
            messages,
            temperature: 0.3,
            max_tokens: Some(1000),
        };

        let response = self.client
            .post(&format!("{}/chat/completions", self.config.openai_base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AiError::ConfigError(format!("API error: {}", error_text)));
        }

        let openai_response: OpenAiResponse = response.json().await?;
        
        Ok(openai_response.choices
            .first()
            .ok_or_else(|| AiError::ModelError("No response from model".to_string()))?
            .message
            .content
            .clone())
    }
}

#[async_trait::async_trait]
impl AiService for OpenAiService {
    async fn analyze_document(&self, content: &str, document_type: &str) -> Result<AnalysisResponse, AiError> {
        let start_time = Instant::now();
        
        let system_message = OpenAiMessage {
            role: "system".to_string(),
            content: format!(
                "You are a legal document analysis expert. Analyze the following {} document and provide structured insights about its legal significance, key requirements, and any potential issues.",
                document_type
            ),
        };

        let user_message = OpenAiMessage {
            role: "user".to_string(),
            content: content.to_string(),
        };

        let response_content = self.create_completion(vec![system_message, user_message]).await?;
        
        let processing_time = start_time.elapsed().as_millis();

        // Create insights from the response
        let insights = vec![
            AiInsight {
                insight_type: InsightType::DocumentAnalysis,
                confidence_score: 0.8,
                data: serde_json::json!({
                    "analysis": response_content,
                    "document_type": document_type,
                    "content_length": content.len()
                }),
                generated_by: "openai_gpt".to_string(),
                created_at: Utc::now(),
            }
        ];

        Ok(AnalysisResponse {
            success: true,
            data: Some(serde_json::json!({ "analysis": response_content })),
            insights,
            processing_time_ms: processing_time,
            model_used: self.config.default_model.clone(),
            error_message: None,
        })
    }

    async fn detect_patterns(&self, data: &serde_json::Value) -> Result<Vec<AiInsight>, AiError> {
        let start_time = Instant::now();
        
        let system_message = OpenAiMessage {
            role: "system".to_string(),
            content: "You are a legal pattern detection expert. Analyze the provided data for patterns related to placement denials, violations, and other legal anomalies. Provide structured insights about any patterns detected.".to_string(),
        };

        let user_message = OpenAiMessage {
            role: "user".to_string(),
            content: serde_json::to_string_pretty(data)?,
        };

        let response_content = self.create_completion(vec![system_message, user_message]).await?;

        let insights = vec![
            AiInsight {
                insight_type: InsightType::Pattern,
                confidence_score: 0.75,
                data: serde_json::json!({
                    "pattern_analysis": response_content,
                    "data_processed": true
                }),
                generated_by: "openai_pattern_detector".to_string(),
                created_at: Utc::now(),
            }
        ];

        Ok(insights)
    }

    async fn generate_timeline_events(&self, context: &str) -> Result<Vec<serde_json::Value>, AiError> {
        let system_message = OpenAiMessage {
            role: "system".to_string(),
            content: "You are a legal timeline expert. Based on the provided context, generate relevant timeline events that should be tracked for this legal case. Return a JSON array of timeline events with date, type, title, description, and importance level.".to_string(),
        };

        let user_message = OpenAiMessage {
            role: "user".to_string(),
            content: context.to_string(),
        };

        let response_content = self.create_completion(vec![system_message, user_message]).await?;
        
        // Parse the response as JSON array of timeline events
        match serde_json::from_str::<Vec<serde_json::Value>>(&response_content) {
            Ok(events) => Ok(events),
            Err(_) => {
                // If parsing fails, create a single event with the raw response
                Ok(vec![serde_json::json!({
                    "event_date": Utc::now().format("%Y-%m-%d").to_string(),
                    "event_type": "ai_generated",
                    "event_title": "AI Timeline Analysis",
                    "event_description": response_content,
                    "importance_level": 3
                })])
            }
        }
    }

    async fn assess_risk(&self, placement_denial: &serde_json::Value) -> Result<f64, AiError> {
        let system_message = OpenAiMessage {
            role: "system".to_string(),
            content: "You are a legal risk assessment expert. Analyze the provided placement denial data and return a risk score between 0.0 and 1.0, where 1.0 represents highest legal risk. Consider factors like denial reasons, timing patterns, and potential violations.".to_string(),
        };

        let user_message = OpenAiMessage {
            role: "user".to_string(),
            content: serde_json::to_string_pretty(placement_denial)?,
        };

        let response_content = self.create_completion(vec![system_message, user_message]).await?;
        
        // Try to parse a numeric risk score from the response
        let risk_score = response_content
            .split_whitespace()
            .find_map(|word| word.parse::<f64>().ok())
            .unwrap_or(0.5); // Default to medium risk if parsing fails

        Ok(risk_score.clamp(0.0, 1.0))
    }

    async fn analyze_communication_sentiment(&self, message: &str) -> Result<f64, AiError> {
        let system_message = OpenAiMessage {
            role: "system".to_string(),
            content: "You are a communication sentiment analysis expert. Analyze the sentiment of the provided message and return a score between -1.0 (very negative) and 1.0 (very positive), with 0.0 being neutral.".to_string(),
        };

        let user_message = OpenAiMessage {
            role: "user".to_string(),
            content: message.to_string(),
        };

        let response_content = self.create_completion(vec![system_message, user_message]).await?;
        
        // Try to parse a numeric sentiment score from the response
        let sentiment_score = response_content
            .split_whitespace()
            .find_map(|word| word.parse::<f64>().ok())
            .unwrap_or(0.0); // Default to neutral if parsing fails

        Ok(sentiment_score.clamp(-1.0, 1.0))
    }
}

#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    temperature: f64,
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}
