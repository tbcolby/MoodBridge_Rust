pub mod log_analyzer;
pub mod activity_tracker;
pub mod report_generator;
pub mod email_service;
pub mod scheduler;
pub mod self_improvement;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Core WARP COMMAND configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpCommandConfig {
    pub email_recipient: String,
    pub email_sender: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub warp_log_path: String,
    pub schedule_time: String, // "0 7 * * *" for 7am daily
    pub timezone: String,      // "America/Chicago"
    pub analysis_window_hours: u64, // 24 for daily analysis
    pub learning_enabled: bool,
}

impl Default for WarpCommandConfig {
    fn default() -> Self {
        Self {
            email_recipient: "tbcolby@pm.me".to_string(),
            email_sender: "warpcommand@moodbridge.dev".to_string(),
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            smtp_username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
            warp_log_path: format!("{}/Library/Logs/warp.log", std::env::var("HOME").unwrap_or_default()),
            schedule_time: "0 7 * * *".to_string(), // 7am daily
            timezone: "America/Chicago".to_string(),
            analysis_window_hours: 24,
            learning_enabled: true,
        }
    }
}

/// Development activity summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentActivity {
    pub date: DateTime<Utc>,
    pub commands_executed: Vec<CommandActivity>,
    pub files_modified: Vec<FileActivity>,
    pub project_focus: Vec<ProjectFocus>,
    pub productivity_metrics: ProductivityMetrics,
    pub ai_insights: Vec<ActivityInsight>,
}

/// Individual command activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandActivity {
    pub command: String,
    pub frequency: u32,
    pub first_used: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub success_rate: f64,
    pub context: Vec<String>,
}

/// File modification activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileActivity {
    pub file_path: String,
    pub language: String,
    pub modifications: u32,
    pub lines_added: Option<u32>,
    pub lines_removed: Option<u32>,
    pub first_modified: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

/// Project focus tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFocus {
    pub project_name: String,
    pub project_path: String,
    pub time_spent_minutes: u32,
    pub activity_type: String, // "coding", "debugging", "research", "documentation"
    pub complexity_score: f64,
}

/// Productivity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductivityMetrics {
    pub total_active_time_minutes: u32,
    pub deep_work_sessions: u32,
    pub context_switches: u32,
    pub error_rate: f64,
    pub learning_indicators: Vec<String>,
    pub efficiency_score: f64,
}

/// AI-generated activity insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityInsight {
    pub insight_type: InsightType,
    pub confidence: f64,
    pub message: String,
    pub data: serde_json::Value,
    pub recommendations: Vec<String>,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    ProductivityTrend,
    LearningPattern,
    TechnicalDebt,
    FocusArea,
    OptimizationOpportunity,
    SkillDevelopment,
    ProjectProgress,
    WorkflowEfficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Daily report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub report_date: DateTime<Utc>,
    pub yesterday_summary: DaysSummary,
    pub today_plan: DaysPlan,
    pub insights: Vec<ActivityInsight>,
    pub learning_recommendations: Vec<String>,
    pub productivity_score: f64,
    pub focus_areas: Vec<String>,
    pub technical_debt_alerts: Vec<String>,
    pub celebration_moments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaysSummary {
    pub accomplishments: Vec<String>,
    pub challenges: Vec<String>,
    pub time_distribution: HashMap<String, u32>,
    pub key_metrics: HashMap<String, serde_json::Value>,
    pub learning_moments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaysPlan {
    pub priority_tasks: Vec<PriorityTask>,
    pub learning_goals: Vec<String>,
    pub focus_time_blocks: Vec<FocusBlock>,
    pub optimization_suggestions: Vec<String>,
    pub estimated_complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityTask {
    pub task: String,
    pub estimated_time_minutes: u32,
    pub complexity: f64,
    pub dependencies: Vec<String>,
    pub success_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusBlock {
    pub time_start: String,
    pub time_end: String,
    pub activity: String,
    pub context: String,
}

/// WARP COMMAND errors
#[derive(Debug, thiserror::Error)]
pub enum WarpCommandError {
    #[error("Log parsing error: {0}")]
    LogParsingError(String),
    #[error("Email sending error: {0}")]
    EmailError(String),
    #[error("Analysis error: {0}")]
    AnalysisError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Scheduler error: {0}")]
    SchedulerError(String),
}

/// Main WARP COMMAND service
pub struct WarpCommandService {
    pub config: WarpCommandConfig,
    pub log_analyzer: log_analyzer::WarpLogAnalyzer,
    pub activity_tracker: activity_tracker::ActivityTracker,
    pub report_generator: report_generator::ReportGenerator,
    pub email_service: email_service::EmailService,
    pub self_improvement: self_improvement::SelfImprovementEngine,
}

impl WarpCommandService {
    pub fn new(config: WarpCommandConfig) -> Result<Self, WarpCommandError> {
        let log_analyzer = log_analyzer::WarpLogAnalyzer::new(&config.warp_log_path)?;
        let activity_tracker = activity_tracker::ActivityTracker::new();
        let report_generator = report_generator::ReportGenerator::new();
        let email_service = email_service::EmailService::new(&config)?;
        let self_improvement = self_improvement::SelfImprovementEngine::new();

        Ok(Self {
            config,
            log_analyzer,
            activity_tracker,
            report_generator,
            email_service,
            self_improvement,
        })
    }

    /// Run daily analysis and send report
    pub async fn generate_daily_report(&self) -> Result<DailyReport, WarpCommandError> {
        tracing::info!("Starting WARP COMMAND daily analysis");

        // 1. Analyze recent logs
        let activities = self.log_analyzer.analyze_daily_activity().await?;
        
        // 2. Track development patterns
        let development_activity = self.activity_tracker.process_activities(&activities).await?;
        
        // 3. Generate insights with self-improvement
        let insights = self.self_improvement.generate_insights(&development_activity).await?;
        
        // 4. Create comprehensive report
        let report = self.report_generator.create_daily_report(development_activity, insights).await?;
        
        // 5. Send email report
        self.email_service.send_daily_report(&report).await?;
        
        tracing::info!("WARP COMMAND daily report completed successfully");
        Ok(report)
    }

    /// Start the scheduler for automatic daily reports
    pub async fn start_scheduler(&self) -> Result<(), WarpCommandError> {
        scheduler::start_daily_scheduler(self.config.clone()).await
    }
}
