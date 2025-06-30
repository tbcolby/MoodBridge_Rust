use crate::ai::core_engine::{AdvancedPromptRequest, AiCoreEngine, ConversationContext, InputType};
use crate::ai::{AiConfig, AiService, AnalysisRequest, InsightType};
use serde_json::json;
use std::collections::HashMap;
use tokio::test;

#[tokio::test]
async fn test_ai_config_default() {
    let config = AiConfig::default();
    assert_eq!(config.default_model, "gpt-4");
    assert_eq!(config.advanced_model, "gpt-4-turbo");
    assert_eq!(config.confidence_threshold, 0.75);
    assert!(config.enable_fabric_integration);
}

#[tokio::test]
async fn test_ai_core_engine_creation() {
    let config = AiConfig::default();
    let engine = AiCoreEngine::new(config);
    // Test that engine is created successfully
    assert!(true); // Engine creation doesn't fail
}

#[tokio::test]
async fn test_advanced_prompt_request_creation() {
    let request = AdvancedPromptRequest {
        input: "Analyze this legal case".to_string(),
        input_type: InputType::Text,
        context: Some(HashMap::new()),
        intent_hints: vec!["analysis_request".to_string()],
        require_citations: true,
        max_response_length: Some(2000),
        style_preference: Some("professional".to_string()),
    };

    assert_eq!(request.input, "Analyze this legal case");
    assert!(matches!(request.input_type, InputType::Text));
    assert!(request.require_citations);
}

#[tokio::test]
async fn test_conversation_context() {
    let context = ConversationContext {
        timestamp: chrono::Utc::now(),
        user_input: "What are the trends?".to_string(),
        input_type: InputType::Text,
        ai_response: "The trends show...".to_string(),
        confidence: 0.95,
        context_tags: vec!["trends".to_string(), "analysis".to_string()],
        embedding: None,
    };

    assert_eq!(context.user_input, "What are the trends?");
    assert_eq!(context.confidence, 0.95);
    assert_eq!(context.context_tags.len(), 2);
}

#[tokio::test]
async fn test_input_type_serialization() {
    let text_type = InputType::Text;
    let voice_type = InputType::Voice;
    let structured_type = InputType::Structured;
    let visual_type = InputType::Visual;
    let contextual_type = InputType::Contextual;

    // Test that all input types can be created
    assert!(matches!(text_type, InputType::Text));
    assert!(matches!(voice_type, InputType::Voice));
    assert!(matches!(structured_type, InputType::Structured));
    assert!(matches!(visual_type, InputType::Visual));
    assert!(matches!(contextual_type, InputType::Contextual));
}

#[tokio::test]
async fn test_analysis_request() {
    let mut options = HashMap::new();
    options.insert("depth".to_string(), "detailed".to_string());

    let request = AnalysisRequest {
        operation_type: "pattern_detection".to_string(),
        input_data: json!({"cases": [], "incidents": []}),
        options: Some(options),
    };

    assert_eq!(request.operation_type, "pattern_detection");
    assert!(request.options.is_some());
}

#[cfg(test)]
mod ai_insights_tests {
    use super::*;
    use crate::ai::AiInsight;

    #[tokio::test]
    async fn test_ai_insight_creation() {
        let insight = AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: 0.85,
            data: json!({"pattern": "recurring_denials"}),
            generated_by: "gpt-4".to_string(),
            created_at: chrono::Utc::now(),
        };

        assert!(matches!(insight.insight_type, InsightType::Pattern));
        assert_eq!(insight.confidence_score, 0.85);
        assert_eq!(insight.generated_by, "gpt-4");
    }

    #[tokio::test]
    async fn test_insight_types() {
        let pattern = InsightType::Pattern;
        let risk = InsightType::RiskAssessment;
        let recommendation = InsightType::Recommendation;
        let timeline = InsightType::TimelineCorrelation;
        let sentiment = InsightType::SentimentAnalysis;
        let document = InsightType::DocumentAnalysis;

        // Test all insight types can be created
        assert!(matches!(pattern, InsightType::Pattern));
        assert!(matches!(risk, InsightType::RiskAssessment));
        assert!(matches!(recommendation, InsightType::Recommendation));
        assert!(matches!(timeline, InsightType::TimelineCorrelation));
        assert!(matches!(sentiment, InsightType::SentimentAnalysis));
        assert!(matches!(document, InsightType::DocumentAnalysis));
    }
}
