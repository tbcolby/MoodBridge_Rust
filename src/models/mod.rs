// use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod requests;

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

// Project Management Models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String, // planning, active, paused, completed, cancelled
    pub priority: String, // critical, high, medium, low
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub completion_date: Option<String>,
    pub progress_percentage: f64,
    pub project_type: String, // security, feature, infrastructure, documentation
    pub owner: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub tags: Option<String>, // JSON array of tags
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String, // todo, in_progress, review, testing, done, blocked
    pub priority: String, // critical, high, medium, low
    pub task_type: String, // implementation, testing, documentation, bug_fix, research
    pub assignee: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub due_date: Option<String>,
    pub completion_date: Option<String>,
    pub blocked_reason: Option<String>,
    pub dependencies: Option<String>, // JSON array of task IDs
    pub labels: Option<String>, // JSON array of labels
    pub ai_priority_score: Option<f64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Milestone {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub target_date: String,
    pub completion_date: Option<String>,
    pub status: String, // upcoming, active, completed, missed
    pub milestone_type: String, // phase, release, deadline, review
    pub success_criteria: Option<String>, // JSON array of criteria
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectDependency {
    pub id: i64,
    pub dependent_project_id: i64,
    pub dependency_project_id: i64,
    pub dependency_type: String, // blocks, requires, enhances
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WorkSession {
    pub id: i64,
    pub task_id: i64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub session_type: String, // focused, research, debugging, testing, documentation
    pub productivity_score: Option<i32>, // 1-10 scale
    pub created_at: Option<String>,
}

// Project Analytics Models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectSummary {
    pub total_projects: i64,
    pub active_projects: i64,
    pub completed_projects: i64,
    pub overdue_tasks: i64,
    pub critical_tasks: i64,
    pub total_estimated_hours: Option<f64>,
    pub total_actual_hours: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TasksByStatus {
    pub status: String,
    pub task_count: i64,
    pub estimated_hours: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectProgress {
    pub project_id: i64,
    pub project_name: String,
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub progress_percentage: f64,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub days_remaining: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductivityMetrics {
    pub date: String,
    pub hours_worked: f64,
    pub tasks_completed: i64,
    pub avg_productivity_score: Option<f64>,
    pub focus_sessions: i64,
}
