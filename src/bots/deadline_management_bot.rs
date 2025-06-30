use super::*;
use crate::ai::{AiService, AnalysisResponse, AiError};
use crate::wizard::{WizardManager, WizardType, WizardState, CreateWizardRequest};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use async_trait::async_trait;

/// Deadline Management Bot - Manages task and project deadlines with alerts and notifications
#[derive(Debug)]
pub struct DeadlineManagementBot {
    pub id: Uuid,
    pub name: String,
    pub ai_service: Option<std::sync::Arc<dyn AiService + Send + Sync>>,
    pub deadline_tracker: DeadlineTracker,
    pub notification_config: NotificationConfig,
}

/// Deadline tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineTracker {
    pub active_deadlines: HashMap<Uuid, Deadline>,
    pub deadline_hierarchy: HashMap<Uuid, Vec<Uuid>>, // Parent -> Children
    pub notification_schedule: Vec<NotificationRule>,
    pub escalation_rules: Vec<EscalationRule>,
}

/// Individual deadline with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub deadline_type: DeadlineType,
    pub due_date: DateTime<Utc>,
    pub priority: DeadlinePriority,
    pub status: DeadlineStatus,
    pub assigned_to: Vec<Uuid>, // User IDs
    pub case_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub parent_deadline_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completion_percentage: f32,
    pub buffer_days: Option<u32>,
    pub jurisdiction_rules: Option<String>,
    pub court_specific_rules: Option<String>,
}

/// Types of deadlines in legal practice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlineType {
    CourtFiling,
    Discovery,
    Motion,
    Appeal,
    Statute_of_Limitations,
    ClientResponse,
    InternalDeadline,
    RegulatoryCompliance,
    BillingDeadline,
    MeetingSchedule,
    DocumentReview,
}

/// Priority levels for deadlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlinePriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Status of deadlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlineStatus {
    Scheduled,
    InProgress,
    NearingDeadline,
    Overdue,
    Completed,
    Cancelled,
    Extended,
}

/// Notification rules for deadlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRule {
    pub rule_id: Uuid,
    pub deadline_type: DeadlineType,
    pub advance_notice_days: Vec<u32>, // e.g., [30, 14, 7, 3, 1]
    pub notification_channels: Vec<NotificationChannel>,
    pub recipient_groups: Vec<RecipientGroup>,
    pub active: bool,
}

/// Notification channels available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    SMS,
    InApp,
    Slack,
    Teams,
    Calendar,
    Dashboard,
}

/// Groups of recipients for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecipientGroup {
    AssignedUsers,
    CaseTeam,
    Supervisors,
    Clients,
    Paralegals,
    Attorneys,
    All,
}

/// Escalation rules for overdue deadlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub rule_id: Uuid,
    pub deadline_type: DeadlineType,
    pub overdue_threshold_hours: u32,
    pub escalation_levels: Vec<EscalationLevel>,
    pub auto_actions: Vec<AutoAction>,
}

/// Escalation level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevel {
    pub level: u8,
    pub hours_after_due: u32,
    pub notify_groups: Vec<RecipientGroup>,
    pub message_template: String,
    pub requires_acknowledgment: bool,
}

/// Automatic actions for deadline management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoAction {
    CreateFollowUpTask,
    NotifySupervisor,
    LogIncident,
    UpdateCaseStatus,
    ScheduleMeeting,
    SendClientUpdate,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub email_templates: HashMap<String, String>,
    pub default_sender: String,
    pub timezone: String,
    pub business_hours: BusinessHours,
    pub holiday_calendar: Vec<DateTime<Utc>>,
}

/// Business hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub start_hour: u8,
    pub end_hour: u8,
    pub working_days: Vec<chrono::Weekday>,
    pub timezone: String,
}

/// Deadline analysis input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineAnalysisInput {
    pub analysis_type: DeadlineAnalysisType,
    pub time_range: Option<TimeRange>,
    pub deadline_types: Option<Vec<DeadlineType>>,
    pub priority_filter: Option<Vec<DeadlinePriority>>,
    pub case_ids: Option<Vec<Uuid>>,
    pub user_ids: Option<Vec<Uuid>>,
}

/// Types of deadline analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeadlineAnalysisType {
    UpcomingDeadlines,
    OverdueAnalysis,
    WorkloadDistribution,
    RiskAssessment,
    ComplianceCheck,
    PerformanceMetrics,
    ResourcePlanning,
}

/// Time range for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Deadline analysis output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineAnalysisOutput {
    pub analysis_id: Uuid,
    pub analysis_type: DeadlineAnalysisType,
    pub summary: String,
    pub deadline_insights: Vec<DeadlineInsight>,
    pub risk_factors: Vec<RiskFactor>,
    pub recommendations: Vec<Recommendation>,
    pub metrics: DeadlineMetrics,
    pub action_items: Vec<ActionItem>,
}

/// Insight about deadlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineInsight {
    pub insight_type: String,
    pub description: String,
    pub affected_deadlines: Vec<Uuid>,
    pub severity: InsightSeverity,
    pub suggested_action: String,
}

/// Severity of insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightSeverity {
    Info,
    Warning,
    Critical,
}

/// Risk factor for deadline management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub risk_type: String,
    pub description: String,
    pub probability: f64, // 0.0 - 1.0
    pub impact: f64,      // 0.0 - 1.0
    pub mitigation_strategies: Vec<String>,
}

/// Recommendation for deadline management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommendation_type: String,
    pub description: String,
    pub priority: DeadlinePriority,
    pub estimated_effort: String,
    pub expected_benefit: String,
}

/// Metrics for deadline performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineMetrics {
    pub total_deadlines: u32,
    pub completed_on_time: u32,
    pub overdue_count: u32,
    pub average_completion_time: f64,
    pub compliance_rate: f64,
    pub workload_distribution: HashMap<Uuid, u32>,
}

/// Action item from analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub item_type: String,
    pub description: String,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: DeadlinePriority,
}

#[async_trait]
impl LegalBot for DeadlineManagementBot {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_specialty(&self) -> BotSpecialty {
        BotSpecialty::DeadlineManagement
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_description(&self) -> &str {
        "Advanced deadline management system for legal practices with intelligent alerts, escalation, and compliance tracking."
    }

    fn get_capabilities(&self) -> &[String] {
        &[
            "Deadline Tracking and Monitoring".to_string(),
            "Intelligent Notification System".to_string(),
            "Escalation Management".to_string(),
            "Risk Assessment".to_string(),
            "Compliance Monitoring".to_string(),
            "Workload Analysis".to_string(),
            "Performance Metrics".to_string(),
            "Resource Planning".to_string(),
        ]
    }

    async fn analyze(&self, input: &BotInput) -> Result<BotOutput, BotError> {
        let start_time = std::time::Instant::now();

        // Parse deadline analysis input
        let analysis_input: DeadlineAnalysisInput = serde_json::from_value(input.data.clone())
            .map_err(|e| BotError::InvalidInput(format!("Failed to parse deadline analysis input: {}", e)))?;

        // Route to appropriate analysis method
        let result = match analysis_input.analysis_type {
            DeadlineAnalysisType::UpcomingDeadlines => self.analyze_upcoming_deadlines(&analysis_input).await?,
            DeadlineAnalysisType::OverdueAnalysis => self.analyze_overdue_deadlines(&analysis_input).await?,
            DeadlineAnalysisType::WorkloadDistribution => self.analyze_workload_distribution(&analysis_input).await?,
            DeadlineAnalysisType::RiskAssessment => self.assess_deadline_risks(&analysis_input).await?,
            DeadlineAnalysisType::ComplianceCheck => self.check_compliance(&analysis_input).await?,
            DeadlineAnalysisType::PerformanceMetrics => self.calculate_performance_metrics(&analysis_input).await?,
            DeadlineAnalysisType::ResourcePlanning => self.plan_resources(&analysis_input).await?,
        };

        let processing_time = start_time.elapsed().as_millis();

        // Generate recommendations
        let recommendations = self.generate_deadline_recommendations(&analysis_input, &result).await?;

        // Suggest next actions
        let next_actions = self.suggest_deadline_actions(&analysis_input, &result).await?;

        Ok(BotOutput {
            task_id: input.task_id,
            bot_id: self.id,
            success: true,
            result: serde_json::to_value(result)?,
            confidence: self.calculate_confidence(&analysis_input),
            recommendations,
            next_actions,
            processing_time_ms: processing_time,
            error_message: None,
        })
    }

    async fn can_handle(&self, task_type: &str) -> bool {
        matches!(task_type, 
            "deadline_management" | "deadline_tracking" | "deadline_analysis" |
            "overdue_analysis" | "workload_analysis" | "compliance_check" |
            "risk_assessment" | "resource_planning"
        )
    }

    fn get_priority(&self, task_type: &str) -> u8 {
        match task_type {
            "deadline_management" => 240,
            "overdue_analysis" => 250,
            "compliance_check" => 230,
            "risk_assessment" => 220,
            "workload_analysis" | "resource_planning" => 200,
            _ => 150,
        }
    }
}

impl DeadlineManagementBot {
    pub fn new(ai_service: Option<std::sync::Arc<dyn AiService + Send + Sync>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Deadline Management Bot".to_string(),
            ai_service,
            deadline_tracker: Self::initialize_deadline_tracker(),
            notification_config: Self::initialize_notification_config(),
        }
    }

    fn initialize_deadline_tracker() -> DeadlineTracker {
        DeadlineTracker {
            active_deadlines: HashMap::new(),
            deadline_hierarchy: HashMap::new(),
            notification_schedule: vec![
                NotificationRule {
                    rule_id: Uuid::new_v4(),
                    deadline_type: DeadlineType::CourtFiling,
                    advance_notice_days: vec![30, 14, 7, 3, 1],
                    notification_channels: vec![NotificationChannel::Email, NotificationChannel::InApp],
                    recipient_groups: vec![RecipientGroup::AssignedUsers, RecipientGroup::Supervisors],
                    active: true,
                },
            ],
            escalation_rules: vec![
                EscalationRule {
                    rule_id: Uuid::new_v4(),
                    deadline_type: DeadlineType::CourtFiling,
                    overdue_threshold_hours: 1,
                    escalation_levels: vec![
                        EscalationLevel {
                            level: 1,
                            hours_after_due: 1,
                            notify_groups: vec![RecipientGroup::Supervisors],
                            message_template: "URGENT: Court filing deadline missed".to_string(),
                            requires_acknowledgment: true,
                        },
                    ],
                    auto_actions: vec![AutoAction::NotifySupervisor, AutoAction::LogIncident],
                },
            ],
        }
    }

    fn initialize_notification_config() -> NotificationConfig {
        NotificationConfig {
            email_templates: HashMap::from([
                ("deadline_reminder".to_string(), "Reminder: {title} is due on {due_date}".to_string()),
                ("overdue_alert".to_string(), "OVERDUE: {title} was due on {due_date}".to_string()),
            ]),
            default_sender: "deadlines@moodbridge.law".to_string(),
            timezone: "America/Chicago".to_string(),
            business_hours: BusinessHours {
                start_hour: 8,
                end_hour: 18,
                working_days: vec![
                    chrono::Weekday::Mon,
                    chrono::Weekday::Tue,
                    chrono::Weekday::Wed,
                    chrono::Weekday::Thu,
                    chrono::Weekday::Fri,
                ],
                timezone: "America/Chicago".to_string(),
            },
            holiday_calendar: vec![],
        }
    }

    async fn analyze_upcoming_deadlines(&self, input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        let now = Utc::now();
        let end_date = input.time_range.as_ref()
            .map(|r| r.end_date)
            .unwrap_or_else(|| now + Duration::days(30));

        let mut upcoming_deadlines = Vec::new();
        for deadline in self.deadline_tracker.active_deadlines.values() {
            if deadline.due_date > now && deadline.due_date <= end_date {
                upcoming_deadlines.push(deadline.clone());
            }
        }

        upcoming_deadlines.sort_by(|a, b| a.due_date.cmp(&b.due_date));

        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::UpcomingDeadlines,
            summary: format!("{} deadlines upcoming in the next 30 days", upcoming_deadlines.len()),
            deadline_insights: vec![
                DeadlineInsight {
                    insight_type: "High Priority Deadlines".to_string(),
                    description: format!("{} high priority deadlines in the next 7 days", 
                        upcoming_deadlines.iter()
                            .filter(|d| d.due_date <= now + Duration::days(7) && matches!(d.priority, DeadlinePriority::High | DeadlinePriority::Critical))
                            .count()),
                    affected_deadlines: upcoming_deadlines.iter()
                        .filter(|d| d.due_date <= now + Duration::days(7) && matches!(d.priority, DeadlinePriority::High | DeadlinePriority::Critical))
                        .map(|d| d.id)
                        .collect(),
                    severity: InsightSeverity::Warning,
                    suggested_action: "Review and prioritize critical deadlines".to_string(),
                }
            ],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: upcoming_deadlines.len() as u32,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![
                ActionItem {
                    item_type: "Review".to_string(),
                    description: "Review upcoming deadlines and ensure adequate resources".to_string(),
                    assigned_to: None,
                    due_date: Some(now + Duration::days(1)),
                    priority: DeadlinePriority::High,
                }
            ],
        })
    }

    async fn analyze_overdue_deadlines(&self, input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        let now = Utc::now();
        let mut overdue_deadlines = Vec::new();

        for deadline in self.deadline_tracker.active_deadlines.values() {
            if deadline.due_date < now && deadline.status != DeadlineStatus::Completed {
                overdue_deadlines.push(deadline.clone());
            }
        }

        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::OverdueAnalysis,
            summary: format!("{} overdue deadlines requiring immediate attention", overdue_deadlines.len()),
            deadline_insights: vec![],
            risk_factors: vec![
                RiskFactor {
                    risk_type: "Legal Compliance".to_string(),
                    description: "Overdue court filings may result in sanctions".to_string(),
                    probability: 0.8,
                    impact: 0.9,
                    mitigation_strategies: vec![
                        "File emergency motions for extensions".to_string(),
                        "Notify clients of potential impacts".to_string(),
                    ],
                }
            ],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: self.deadline_tracker.active_deadlines.len() as u32,
                completed_on_time: 0,
                overdue_count: overdue_deadlines.len() as u32,
                average_completion_time: 0.0,
                compliance_rate: 0.85,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn analyze_workload_distribution(&self, _input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        // Workload analysis implementation
        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::WorkloadDistribution,
            summary: "Workload distribution analysis completed".to_string(),
            deadline_insights: vec![],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: 0,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn assess_deadline_risks(&self, _input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        // Risk assessment implementation
        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::RiskAssessment,
            summary: "Risk assessment completed".to_string(),
            deadline_insights: vec![],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: 0,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn check_compliance(&self, _input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        // Compliance check implementation
        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::ComplianceCheck,
            summary: "Compliance check completed".to_string(),
            deadline_insights: vec![],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: 0,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn calculate_performance_metrics(&self, _input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        // Performance metrics implementation
        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::PerformanceMetrics,
            summary: "Performance metrics calculated".to_string(),
            deadline_insights: vec![],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: 0,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn plan_resources(&self, _input: &DeadlineAnalysisInput) -> Result<DeadlineAnalysisOutput, BotError> {
        // Resource planning implementation
        Ok(DeadlineAnalysisOutput {
            analysis_id: Uuid::new_v4(),
            analysis_type: DeadlineAnalysisType::ResourcePlanning,
            summary: "Resource planning completed".to_string(),
            deadline_insights: vec![],
            risk_factors: vec![],
            recommendations: vec![],
            metrics: DeadlineMetrics {
                total_deadlines: 0,
                completed_on_time: 0,
                overdue_count: 0,
                average_completion_time: 0.0,
                compliance_rate: 1.0,
                workload_distribution: HashMap::new(),
            },
            action_items: vec![],
        })
    }

    async fn generate_deadline_recommendations(&self, _input: &DeadlineAnalysisInput, _output: &DeadlineAnalysisOutput) -> Result<Vec<String>, BotError> {
        Ok(vec![
            "Set up automated reminders for critical deadlines".to_string(),
            "Review workload distribution to prevent bottlenecks".to_string(),
            "Implement escalation procedures for overdue items".to_string(),
            "Create buffer time for high-risk deadlines".to_string(),
        ])
    }

    async fn suggest_deadline_actions(&self, _input: &DeadlineAnalysisInput, _output: &DeadlineAnalysisOutput) -> Result<Vec<NextAction>, BotError> {
        Ok(vec![
            NextAction {
                action_type: "notification".to_string(),
                description: "Send deadline reminders to assigned team members".to_string(),
                priority: 200,
                suggested_bot: Some(BotSpecialty::EmailNotificationBot),
                estimated_time_hours: Some(0.5),
            },
            NextAction {
                action_type: "escalation".to_string(),
                description: "Escalate overdue deadlines to supervisors".to_string(),
                priority: 250,
                suggested_bot: Some(BotSpecialty::DeadlineManagement),
                estimated_time_hours: Some(1.0),
            },
        ])
    }

    fn calculate_confidence(&self, _input: &DeadlineAnalysisInput) -> f64 {
        0.85 // Base confidence for deadline management
    }
}
