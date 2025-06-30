use moodbridge_rust::ai::analytics::*;
use serde_json::json;

#[tokio::test]
async fn test_analyze_placement_patterns() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({
            "id": 1,
            "denied_date": "2024-01-15",
            "duration_hours": 8.0,
            "denial_reason": "Scheduling conflict",
            "ai_risk_score": 0.8
        }),
        json!({
            "id": 2,
            "denied_date": "2024-01-20",
            "duration_hours": 12.0,
            "denial_reason": "Medical issue",
            "ai_risk_score": 0.6
        }),
        json!({
            "id": 3,
            "denied_date": "2024-02-01",
            "duration_hours": 4.0,
            "denial_reason": "Scheduling conflict",
            "ai_risk_score": 0.9
        }),
    ];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();
    assert!(!result.is_empty());
    assert!(result.len() >= 2); // Should have frequency and duration insights
}

#[tokio::test]
async fn test_analyze_placement_patterns_empty() {
    let analytics = LegalAnalytics;
    let denials = vec![];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_analyze_communication_patterns() {
    let analytics = LegalAnalytics;
    let communications = vec![
        json!({
            "communication_date": "2024-01-15",
            "message": "Requesting placement update",
            "sender": "parent"
        }),
        json!({
            "communication_date": "2024-01-16",
            "message": "Placement denied",
            "sender": "agency"
        }),
    ];

    let result = LegalAnalytics::analyze_communication_patterns(&communications).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].generated_by, "legal_analytics");
}

#[tokio::test]
async fn test_analyze_communication_patterns_empty() {
    let analytics = LegalAnalytics;
    let communications = vec![];

    let result = LegalAnalytics::analyze_communication_patterns(&communications).unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_generate_case_statistics() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({
            "duration_hours": 8.0,
            "ai_risk_score": 0.8
        }),
        json!({
            "duration_hours": 12.0,
            "ai_risk_score": 0.9
        }),
    ];
    let communications = vec![json!({"type": "email"})];
    let timeline_events = vec![json!({"event": "placement_request"})];

    let result =
        LegalAnalytics::generate_case_statistics(&denials, &communications, &timeline_events)
            .unwrap();

    assert_eq!(result.generated_by, "case_statistics_analyzer");
    assert!(result.confidence_score > 0.8);

    let data = result.data.as_object().unwrap();
    assert!(data.contains_key("case_overview"));
    assert!(data.contains_key("risk_assessment"));
    assert!(data.contains_key("trends"));
}

#[tokio::test]
async fn test_denial_frequency_analysis() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({"denied_date": "2024-01-15"}),
        json!({"denied_date": "2024-01-20"}),
        json!({"denied_date": "2024-02-05"}),
    ];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();
    let frequency_insight = result
        .iter()
        .find(|i| i.data.get("pattern_type").and_then(|t| t.as_str()) == Some("denial_frequency"))
        .unwrap();

    assert_eq!(frequency_insight.generated_by, "frequency_analyzer");
    assert!(frequency_insight.confidence_score >= 0.8);
}

#[tokio::test]
async fn test_duration_patterns_analysis() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({"duration_hours": 8.0}),
        json!({"duration_hours": 12.0}),
        json!({"duration_hours": 6.0}),
    ];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();
    let duration_insight = result
        .iter()
        .find(|i| i.data.get("pattern_type").and_then(|t| t.as_str()) == Some("duration_analysis"))
        .unwrap();

    assert_eq!(duration_insight.generated_by, "duration_analyzer");
    assert!(duration_insight.confidence_score >= 0.85);

    let data = duration_insight.data.as_object().unwrap();
    assert!(data.contains_key("total_lost_hours"));
    assert!(data.contains_key("average_duration"));
    assert!(data.contains_key("max_duration"));
    assert!(data.contains_key("min_duration"));
}

#[tokio::test]
async fn test_denial_reasons_analysis() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({"denial_reason": "Scheduling conflict"}),
        json!({"denial_reason": "Medical issue"}),
        json!({"denial_reason": "Scheduling conflict"}),
    ];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();
    let reason_insight = result
        .iter()
        .find(|i| i.data.get("pattern_type").and_then(|t| t.as_str()) == Some("denial_reasons"))
        .unwrap();

    assert_eq!(reason_insight.generated_by, "reason_analyzer");
    assert!(reason_insight.confidence_score >= 0.7);

    let data = reason_insight.data.as_object().unwrap();
    assert!(data.contains_key("reason_breakdown"));
    assert!(data.contains_key("most_common_reason"));
    assert!(data.contains_key("unique_reasons"));
}

#[tokio::test]
async fn test_high_risk_assessment() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({"ai_risk_score": 0.9}),
        json!({"ai_risk_score": 0.8}),
        json!({"ai_risk_score": 0.3}),
    ];
    let communications = vec![];
    let timeline_events = vec![];

    let result =
        LegalAnalytics::generate_case_statistics(&denials, &communications, &timeline_events)
            .unwrap();

    let risk_data = result.data["risk_assessment"].as_object().unwrap();
    assert_eq!(risk_data["overall_risk_level"].as_str().unwrap(), "HIGH");

    let risk_percentage = risk_data["risk_percentage"].as_f64().unwrap();
    assert!(risk_percentage > 50.0); // 2 out of 3 are high risk
}

#[tokio::test]
async fn test_no_duration_data_error() {
    let analytics = LegalAnalytics;
    let denials = vec![
        json!({"denied_date": "2024-01-15"}), // No duration_hours
        json!({"denied_date": "2024-01-20"}),
    ];

    let result = LegalAnalytics::analyze_placement_patterns(&denials).unwrap();

    // Should have frequency insight but not duration insight
    let has_frequency = result
        .iter()
        .any(|i| i.data.get("pattern_type").and_then(|t| t.as_str()) == Some("denial_frequency"));
    let has_duration = result
        .iter()
        .any(|i| i.data.get("pattern_type").and_then(|t| t.as_str()) == Some("duration_analysis"));

    assert!(has_frequency);
    assert!(!has_duration); // Should not have duration analysis without duration data
}

#[tokio::test]
async fn test_communication_frequency_calculation() {
    let analytics = LegalAnalytics;
    let communications = vec![
        json!({"communication_date": "2024-01-15"}),
        json!({"communication_date": "2024-01-20"}),
        json!({"communication_date": "2024-02-05"}),
    ];

    let result = LegalAnalytics::analyze_communication_patterns(&communications).unwrap();
    assert_eq!(result.len(), 1);

    let insight = &result[0];
    let frequency_data = insight.data["frequency_analysis"].as_object().unwrap();
    assert!(frequency_data.contains_key("2024-01"));
    assert!(frequency_data.contains_key("2024-02"));
}
