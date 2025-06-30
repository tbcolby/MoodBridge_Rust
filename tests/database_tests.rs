use crate::db::{create_pool, run_migrations, seed_sample_data};
use crate::models::{
    CaseInfo, Communication, DenialSummary, Exhibit, MonthlyTrend, PlacementDenial, TimelineEvent,
    Violation,
};
use sqlx::{sqlite::SqlitePool, Row};
use tokio::test;

#[tokio::test]
async fn test_database_connection() {
    let pool = create_pool("sqlite::memory:").await;
    assert!(pool.is_ok());
}

#[tokio::test]
async fn test_database_migrations() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    let result = run_migrations(&pool).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_sample_data_seeding() {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    run_migrations(&pool).await.unwrap();
    let result = seed_sample_data(&pool).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_placement_denial_query() {
    let pool = setup_test_database().await;

    let denials = sqlx::query_as::<_, PlacementDenial>("SELECT * FROM placement_denials LIMIT 5")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(!denials.is_empty());
    assert!(denials[0].id > 0);
    assert!(!denials[0].denied_date.is_empty());
}

#[tokio::test]
async fn test_timeline_events_query() {
    let pool = setup_test_database().await;

    let events =
        sqlx::query_as::<_, TimelineEvent>("SELECT * FROM timeline_events ORDER BY event_date")
            .fetch_all(&pool)
            .await
            .unwrap();

    assert!(!events.is_empty());
    assert!(!events[0].event_title.is_empty());
}

#[tokio::test]
async fn test_violations_query() {
    let pool = setup_test_database().await;

    let violations = sqlx::query_as::<_, Violation>("SELECT * FROM violations")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(!violations.is_empty());
    assert!(violations[0].impact_score.is_some());
}

#[tokio::test]
async fn test_dashboard_statistics() {
    let pool = setup_test_database().await;

    let stats = sqlx::query(
        "SELECT 
            COUNT(*) as total_incidents,
            COALESCE(SUM(duration_hours), 0) as total_hours,
            COALESCE(AVG(duration_hours), 0) as avg_duration
         FROM placement_denials",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let total_incidents: i64 = stats.get("total_incidents");
    let total_hours: f64 = stats.get("total_hours");
    let avg_duration: f64 = stats.get("avg_duration");

    assert!(total_incidents > 0);
    assert!(total_hours > 0.0);
    assert!(avg_duration > 0.0);
}

#[tokio::test]
async fn test_monthly_trends() {
    let pool = setup_test_database().await;

    let trends = sqlx::query(
        "SELECT 
            substr(denied_date, 1, 7) as month,
            COUNT(*) as count
         FROM placement_denials 
         GROUP BY substr(denied_date, 1, 7)
         ORDER BY month",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(!trends.is_empty());
    for trend in trends {
        let month: String = trend.get("month");
        let count: i64 = trend.get("count");
        assert!(!month.is_empty());
        assert!(count > 0);
    }
}

#[tokio::test]
async fn test_violation_categories() {
    let pool = setup_test_database().await;

    let categories = sqlx::query(
        "SELECT 
            violation_category as category,
            COUNT(*) as count
         FROM placement_denials 
         WHERE violation_category IS NOT NULL
         GROUP BY violation_category
         ORDER BY count DESC",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(!categories.is_empty());
    for category in categories {
        let cat_name: String = category.get("category");
        let count: i64 = category.get("count");
        assert!(!cat_name.is_empty());
        assert!(count > 0);
    }
}

#[tokio::test]
async fn test_recent_incidents() {
    let pool = setup_test_database().await;

    let recent = sqlx::query_as::<_, PlacementDenial>(
        "SELECT * FROM placement_denials 
         ORDER BY denied_date DESC 
         LIMIT 10",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(recent.len() <= 10);
    assert!(!recent.is_empty());

    // Check that dates are in descending order
    for i in 1..recent.len() {
        assert!(recent[i - 1].denied_date >= recent[i].denied_date);
    }
}

#[tokio::test]
async fn test_database_constraints() {
    let pool = setup_test_database().await;

    // Test inserting duplicate data should not fail (no unique constraints)
    let result = sqlx::query(
        "INSERT INTO placement_denials 
         (denied_date, duration_hours, denial_reason) 
         VALUES (?, ?, ?)",
    )
    .bind("2024-01-01")
    .bind(5.0)
    .bind("Test reason")
    .execute(&pool)
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_case_info_model() {
    let case = CaseInfo {
        id: 1,
        docket_number: "2024-CV-001".to_string(),
        case_title: "Test Case".to_string(),
        court: "District Court".to_string(),
        status: Some("Active".to_string()),
        created_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(case.id, 1);
    assert_eq!(case.docket_number, "2024-CV-001");
    assert_eq!(case.case_title, "Test Case");
}

#[tokio::test]
async fn test_communication_model() {
    let comm = Communication {
        id: 1,
        communication_date: "2024-01-01".to_string(),
        sender: Some("John Doe".to_string()),
        recipient: Some("Jane Smith".to_string()),
        medium: Some("Email".to_string()),
        subject: Some("Legal Matter".to_string()),
        message_content: Some("Important message".to_string()),
        related_to_placement: Some(true),
        created_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(comm.id, 1);
    assert_eq!(comm.communication_date, "2024-01-01");
    assert_eq!(comm.related_to_placement, Some(true));
}

#[tokio::test]
async fn test_exhibit_model() {
    let exhibit = Exhibit {
        id: 1,
        exhibit_label: Some("A".to_string()),
        document_name: "Evidence Document".to_string(),
        file_path: Some("/path/to/file.pdf".to_string()),
        file_size_bytes: Some(1024),
        media_type: Some("application/pdf".to_string()),
        hash_sha256: Some("abc123".to_string()),
        description: Some("Important evidence".to_string()),
        category: Some("Legal".to_string()),
        created_at: Some("2024-01-01".to_string()),
    };

    assert_eq!(exhibit.id, 1);
    assert_eq!(exhibit.document_name, "Evidence Document");
    assert_eq!(exhibit.file_size_bytes, Some(1024));
}

// Helper function to set up test database
async fn setup_test_database() -> SqlitePool {
    let pool = create_pool("sqlite::memory:").await.unwrap();
    run_migrations(&pool).await.unwrap();
    seed_sample_data(&pool).await.unwrap();
    pool
}

#[cfg(test)]
mod model_serialization_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_placement_denial_serialization() {
        let denial = PlacementDenial {
            id: 1,
            denied_date: "2024-01-01".to_string(),
            requested_start_time: Some("10:00".to_string()),
            requested_end_time: Some("15:00".to_string()),
            duration_hours: Some(5.0),
            denial_reason: Some("Schedule conflict".to_string()),
            violation_category: Some("Scheduling Issue".to_string()),
            evidence_attached: Some("Email".to_string()),
            created_at: Some("2024-01-01".to_string()),
        };

        let json = serde_json::to_string(&denial).unwrap();
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("Schedule conflict"));
    }

    #[test]
    fn test_timeline_event_serialization() {
        let event = TimelineEvent {
            id: 1,
            event_date: "2024-01-01".to_string(),
            event_type: Some("court".to_string()),
            event_title: "Court Hearing".to_string(),
            event_description: Some("Initial hearing".to_string()),
            importance_level: Some(5),
            created_at: Some("2024-01-01".to_string()),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Court Hearing"));
        assert!(json.contains("court"));
    }
}
