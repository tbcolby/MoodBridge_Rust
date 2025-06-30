use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CaseInfo {
    pub id: i64,
    pub docket_number: String,
    pub case_title: String,
    pub court: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PlacementDenial {
    pub id: i64,
    pub denied_date: String,
    pub requested_start_time: Option<String>,
    pub requested_end_time: Option<String>,
    pub duration_hours: Option<f64>,
    pub denial_reason: Option<String>,
    pub violation_category: Option<String>,
    pub evidence_attached: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TimelineEvent {
    pub id: i64,
    pub event_date: String,
    pub event_type: Option<String>,
    pub event_title: String,
    pub event_description: Option<String>,
    pub importance_level: Option<i32>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Exhibit {
    pub id: i64,
    pub exhibit_label: Option<String>,
    pub document_name: String,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub media_type: Option<String>,
    pub hash_sha256: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Communication {
    pub id: i64,
    pub communication_date: String,
    pub sender: Option<String>,
    pub recipient: Option<String>,
    pub medium: Option<String>,
    pub subject: Option<String>,
    pub message_content: Option<String>,
    pub related_to_placement: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Violation {
    pub id: i64,
    pub violation_date: String,
    pub violation_type: Option<String>,
    pub description: Option<String>,
    pub stipulation_reference: Option<String>,
    pub impact_score: Option<i32>,
    pub placement_denial_id: Option<i64>,
    pub created_at: Option<String>,
}

// Dashboard analytics models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DenialSummary {
    pub total_denials: i64,
    pub total_hours_lost: Option<f64>,
    pub months_affected: i64,
    pub avg_hours_per_denial: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MonthlyTrend {
    pub month: String,
    pub denials_count: i64,
    pub hours_lost: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ViolationCategory {
    pub violation_category: Option<String>,
    pub incident_count: i64,
    pub avg_severity: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ComplianceScore {
    pub metric: String,
    pub compliance_percentage: f64,
    pub total_violations: i64,
    pub period: String,
}
