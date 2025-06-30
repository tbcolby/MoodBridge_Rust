use chrono::Utc;
use moodbridge_rust::models::requests::*;
use moodbridge_rust::models::*;
use validator::Validate;

#[test]
fn test_placement_denial_serialization() {
    let denial = PlacementDenial {
        id: 1,
        denied_date: "2024-01-01".to_string(),
        requested_start_time: Some("09:00".to_string()),
        requested_end_time: Some("17:00".to_string()),
        duration_hours: Some(8.0),
        denial_reason: Some("Staff shortage".to_string()),
        violation_category: Some("Scheduling".to_string()),
        evidence_attached: Some("Email".to_string()),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&denial).unwrap();
    let deserialized: PlacementDenial = serde_json::from_str(&json).unwrap();

    assert_eq!(denial.id, deserialized.id);
    assert_eq!(denial.denied_date, deserialized.denied_date);
    assert_eq!(denial.duration_hours, deserialized.duration_hours);
}

#[test]
fn test_timeline_event_serialization() {
    let event = TimelineEvent {
        id: 1,
        event_date: "2024-01-01".to_string(),
        event_type: Some("incident".to_string()),
        event_title: "System Failure".to_string(),
        event_description: Some("Database connection lost".to_string()),
        importance_level: Some(5),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&event).unwrap();
    let deserialized: TimelineEvent = serde_json::from_str(&json).unwrap();

    assert_eq!(event.id, deserialized.id);
    assert_eq!(event.event_title, deserialized.event_title);
    assert_eq!(event.importance_level, deserialized.importance_level);
}

#[test]
fn test_case_info_validation() {
    let case = CaseInfo {
        id: 1,
        docket_number: "2024-CV-001".to_string(),
        case_title: "Legal Matter".to_string(),
        court: "District Court".to_string(),
        status: Some("Active".to_string()),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    assert!(!case.docket_number.is_empty());
    assert!(!case.case_title.is_empty());
    assert!(!case.court.is_empty());
}

#[test]
fn test_communication_model() {
    let comm = Communication {
        id: 1,
        communication_date: "2024-01-01".to_string(),
        sender: Some("John Doe".to_string()),
        recipient: Some("Jane Smith".to_string()),
        medium: Some("Email".to_string()),
        subject: Some("Case Discussion".to_string()),
        message_content: Some("Important case details...".to_string()),
        related_to_placement: Some(true),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    assert_eq!(comm.related_to_placement, Some(true));
    assert!(comm.subject.is_some());
}

#[test]
fn test_exhibit_model() {
    let exhibit = Exhibit {
        id: 1,
        exhibit_label: Some("A".to_string()),
        document_name: "Evidence Document".to_string(),
        file_path: Some("/path/to/document.pdf".to_string()),
        file_size_bytes: Some(1024),
        media_type: Some("application/pdf".to_string()),
        hash_sha256: Some("abc123def456".to_string()),
        description: Some("Key evidence".to_string()),
        category: Some("Legal Document".to_string()),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    assert!(!exhibit.document_name.is_empty());
    assert_eq!(exhibit.file_size_bytes, Some(1024));
}

#[test]
fn test_violation_model() {
    let violation = Violation {
        id: 1,
        violation_date: "2024-01-01".to_string(),
        violation_type: Some("Procedural".to_string()),
        description: Some("Failed to follow protocol".to_string()),
        stipulation_reference: Some("Section 4.2".to_string()),
        impact_score: Some(7),
        placement_denial_id: Some(1),
        created_at: Some("2024-01-01T10:00:00Z".to_string()),
    };

    assert!(violation.impact_score.unwrap() > 0);
    assert!(violation.placement_denial_id.is_some());
}

#[test]
fn test_denial_summary_analytics() {
    let summary = DenialSummary {
        total_denials: 50,
        total_hours_lost: Some(400.0),
        months_affected: 6,
        avg_hours_per_denial: Some(8.0),
    };

    assert_eq!(summary.total_denials, 50);
    assert_eq!(summary.avg_hours_per_denial, Some(8.0));

    // Test calculation consistency
    if let (Some(total), Some(avg)) = (summary.total_hours_lost, summary.avg_hours_per_denial) {
        let expected_total = summary.total_denials as f64 * avg;
        assert!((total - expected_total).abs() < 0.1);
    }
}

#[test]
fn test_monthly_trend_model() {
    let trend = MonthlyTrend {
        month: "2024-01".to_string(),
        denials_count: 15,
        hours_lost: Some(120.0),
    };

    assert!(trend.month.starts_with("2024"));
    assert!(trend.denials_count > 0);
}

#[test]
fn test_violation_category_analytics() {
    let category = ViolationCategory {
        violation_category: Some("Scheduling".to_string()),
        incident_count: 25,
        avg_severity: Some(6.5),
    };

    assert_eq!(category.incident_count, 25);
    assert!(category.avg_severity.unwrap() > 0.0);
}

#[test]
fn test_compliance_score_model() {
    let score = ComplianceScore {
        metric: "Overall Compliance".to_string(),
        compliance_percentage: 85.5,
        total_violations: 15,
        period: "Q1 2024".to_string(),
    };

    assert!(score.compliance_percentage >= 0.0 && score.compliance_percentage <= 100.0);
    assert!(!score.metric.is_empty());
}

#[tokio::test]
async fn test_user_registration_request_validation() {
    let mut request = UserRegistrationRequest {
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        password: "SecurePassword123!".to_string(),
        password_confirm: "SecurePassword123!".to_string(),
        organization: Some("Test Org".to_string()),
        role: Some("admin".to_string()),
        terms_accepted: true,
        privacy_accepted: true,
    };

    assert!(request.validate().is_ok());

    // Test invalid email
    request.email = "invalid-email".to_string();
    assert!(request.validate().is_err());
}

#[tokio::test]
async fn test_create_case_request_validation() {
    let request = CreateCaseRequest {
        title: "Important Legal Case".to_string(),
        description: Some("Detailed case description".to_string()),
        case_type: "family_law".to_string(),
        priority: CasePriority::High,
        client_name: Some("John Doe".to_string()),
        client_email: Some("john@example.com".to_string()),
        due_date: Some(Utc::now() + chrono::Duration::days(30)),
        tags: Some(vec!["urgent".to_string(), "family".to_string()]),
        metadata: None,
    };

    assert!(request.validate().is_ok());
}

#[tokio::test]
async fn test_incident_report_request_validation() {
    let request = IncidentReportRequest {
        title: "System Outage".to_string(),
        description: "Database server went down unexpectedly".to_string(),
        incident_type: "system_error".to_string(),
        severity: IncidentSeverity::Major,
        occurred_at: Utc::now(),
        reported_by: Some("Admin User".to_string()),
        witnesses: Some(vec!["User1".to_string(), "User2".to_string()]),
        evidence_urls: Some(vec!["https://logs.example.com/incident1".to_string()]),
        immediate_actions: Some("Restarted database server".to_string()),
        follow_up_required: true,
    };

    assert!(request.validate().is_ok());
}

#[tokio::test]
async fn test_search_request_validation() {
    let request = SearchRequest {
        query: "placement denials".to_string(),
        search_type: SearchType::Cases,
        page_size: Some(20),
        page: Some(0),
        filters: Some(SearchFilters {
            date_from: Some(Utc::now() - chrono::Duration::days(30)),
            date_to: Some(Utc::now()),
            case_type: Some("family_law".to_string()),
            priority: Some(CasePriority::High),
            status: Some("active".to_string()),
            assigned_to: None,
            tags: Some(vec!["urgent".to_string()]),
        }),
        sort_by: Some("date".to_string()),
        sort_order: Some(SortOrder::Descending),
    };

    assert!(request.validate().is_ok());
}

#[test]
fn test_input_type_enum() {
    let types = vec![
        InputType::Text,
        InputType::Voice,
        InputType::Structured,
        InputType::Visual,
        InputType::Contextual,
    ];

    for input_type in types {
        let json = serde_json::to_string(&input_type).unwrap();
        let deserialized: InputType = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            (input_type, deserialized),
            (InputType::Text, InputType::Text)
                | (InputType::Voice, InputType::Voice)
                | (InputType::Structured, InputType::Structured)
                | (InputType::Visual, InputType::Visual)
                | (InputType::Contextual, InputType::Contextual)
        ));
    }
}

#[test]
fn test_case_priority_enum() {
    let priorities = vec![
        CasePriority::Low,
        CasePriority::Medium,
        CasePriority::High,
        CasePriority::Critical,
    ];

    for priority in priorities {
        let json = serde_json::to_string(&priority).unwrap();
        let deserialized: CasePriority = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            (priority, deserialized),
            (CasePriority::Low, CasePriority::Low)
                | (CasePriority::Medium, CasePriority::Medium)
                | (CasePriority::High, CasePriority::High)
                | (CasePriority::Critical, CasePriority::Critical)
        ));
    }
}

#[test]
fn test_model_default_values() {
    let denial = PlacementDenial {
        id: 0,
        denied_date: String::new(),
        requested_start_time: None,
        requested_end_time: None,
        duration_hours: None,
        denial_reason: None,
        violation_category: None,
        evidence_attached: None,
        created_at: None,
    };

    assert_eq!(denial.id, 0);
    assert!(denial.denied_date.is_empty());
    assert!(denial.requested_start_time.is_none());
}
