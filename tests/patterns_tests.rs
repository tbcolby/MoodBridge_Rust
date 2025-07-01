use moodbridge_rust::ai::patterns::*;
use serde_json::json;

#[tokio::test]
async fn test_detect_placement_denial_patterns() {
    let detector = PatternDetector::new();
    let denials = vec![
        json!({ "denied_date": "2024-01-15", "duration_hours": 8.0 }),
        json!({ "denied_date": "2024-01-16", "duration_hours": 12.0 }),
        json!({ "denied_date": "2024-01-20", "duration_hours": 6.0 }),
    ];

    let result = detector.detect_placement_denial_patterns(&denials).unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_detect_placement_denial_patterns_empty() {
    let detector = PatternDetector::new();
    let denials = vec![];

    let result = detector.detect_placement_denial_patterns(&denials);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detect_communication_patterns() {
    let detector = PatternDetector::new();
    let communications = vec![json!({ "communication_date": "2024-01-15" })];

    let result = detector
        .detect_communication_patterns(&communications)
        .unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_detect_communication_patterns_empty() {
    let detector = PatternDetector::new();
    let communications = vec![];

    let result = detector.detect_communication_patterns(&communications);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detect_timeline_patterns() {
    let detector = PatternDetector::new();
    let events = vec![json!({ "event_date": "2024-01-15", "event_type": "document" })];

    let result = detector.detect_timeline_patterns(&events).unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_detect_timeline_patterns_empty() {
    let detector = PatternDetector::new();
    let events = vec![];

    let result = detector.detect_timeline_patterns(&events);
    assert!(result.is_ok());
}
