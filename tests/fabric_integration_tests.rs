use moodbridge_rust::ai::fabric_integration::*;
use serde_json::json;

#[tokio::test]
async fn test_analyze_legal_document_court_order() {
    let fabric_patterns = LegalFabricPatterns;
    let content = "Court Order: All relevant information here.";

    let result = fabric_patterns
        .analyze_legal_document(content, "court_order")
        .await
        .unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].generated_by, "fabric_legal_assistant");
}

#[tokio::test]
async fn test_analyze_legal_document_empty() {
    let fabric_patterns = LegalFabricPatterns;
    let content = "";

    let result = fabric_patterns
        .analyze_legal_document(content, "court_order")
        .await;

    // Empty content should still work - fabric pattern analysis is simulated
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_extract_violations() {
    let fabric_patterns = LegalFabricPatterns;
    let content = "Possible violations in legal document.";

    let result = fabric_patterns.extract_violations(content).await.unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].generated_by, "fabric_pattern_detector");
}

#[tokio::test]
async fn test_correlate_timeline_events() {
    let fabric_patterns = LegalFabricPatterns;
    let events = vec![
        json!({"event_date": "2024-01-15", "event_type": "document"}),
        json!({"event_date": "2024-01-20", "event_type": "denial"}),
    ];

    let result = fabric_patterns
        .correlate_timeline_events(&events)
        .await
        .unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_analyze_communication_legal_context() {
    let fabric_patterns = LegalFabricPatterns;
    let result = fabric_patterns
        .analyze_communication_legal_context("Message content", "Message context")
        .await
        .unwrap();

    assert!(!result.is_empty());
    assert_eq!(result[0].generated_by, "fabric_legal_assistant");
}
