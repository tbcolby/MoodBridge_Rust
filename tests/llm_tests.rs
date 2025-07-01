use moodbridge_rust::ai::llm::*;
use moodbridge_rust::ai::{AiConfig, AiService};
use serde_json::json;

#[tokio::test]
async fn test_llm_client_creation() {
    let mut config = AiConfig::default();
    config.openai_api_key = Some("dummy_key".to_string());
    config.default_model = "gpt-3".to_string();

    let service = OpenAiService::new(config);

    let result = service.analyze_document("Some content", "general").await;
    assert!(result.is_err()); // API key is invalid
}

#[tokio::test]
async fn test_generate_timeline_events() {
    let mut config = AiConfig::default();
    config.openai_api_key = Some("dummy_key".to_string());
    config.default_model = "gpt-3".to_string();

    let service = OpenAiService::new(config);

    let result = service.generate_timeline_events("Some context").await;
    assert!(result.is_err()); // Invalid key leads to rejection
}

#[tokio::test]
async fn test_assess_risk() {
    let mut config = AiConfig::default();
    config.openai_api_key = Some("dummy_key".to_string());
    config.default_model = "gpt-3".to_string();

    let service = OpenAiService::new(config);
    let placement_denial = json!({ "denial_reason": "Lack of evidence" });

    let result = service.assess_risk(&placement_denial).await;
    assert!(result.is_err()); // Invalid key leads to rejection
}
