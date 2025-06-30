use super::*;
use std::collections::HashMap;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_daily_report(&self, activity: DevelopmentActivity, insights: Vec<ActivityInsight>) -> Result<DailyReport, WarpCommandError> {
        let report_date = Utc::now();
        
        // Create yesterday's summary
        let yesterday_summary = DaysSummary {
            accomplishments: vec![
                "Successfully implemented WARP COMMAND system".to_string(),
                "Analyzed 153,356 log entries from major build day".to_string(),
                "Built comprehensive development activity tracking".to_string(),
            ],
            challenges: vec![
                "Setting up email configuration".to_string(),
            ],
            time_distribution: activity.project_focus.iter()
                .map(|pf| (pf.project_name.clone(), pf.time_spent_minutes))
                .collect(),
            key_metrics: {
                let mut map = HashMap::new();
                map.insert("total_commands".to_string(), serde_json::json!(activity.commands_executed.len()));
                map.insert("files_modified".to_string(), serde_json::json!(activity.files_modified.len()));
                map.insert("productivity_score".to_string(), serde_json::json!(activity.productivity_metrics.efficiency_score));
                map
            },
            learning_moments: activity.productivity_metrics.learning_indicators.clone(),
        };

        // Create today's plan
        let today_plan = DaysPlan {
            priority_tasks: vec![
                PriorityTask {
                    task: "Complete WARP COMMAND email integration".to_string(),
                    estimated_time_minutes: 60,
                    complexity: 6.0,
                    dependencies: vec!["SMTP configuration".to_string()],
                    success_probability: 0.85,
                },
                PriorityTask {
                    task: "Set up automated daily scheduling".to_string(),
                    estimated_time_minutes: 45,
                    complexity: 7.0,
                    dependencies: vec!["Email service".to_string()],
                    success_probability: 0.80,
                },
                PriorityTask {
                    task: "Test end-to-end WARP COMMAND workflow".to_string(),
                    estimated_time_minutes: 30,
                    complexity: 4.0,
                    dependencies: vec!["Email service".to_string(), "Scheduler".to_string()],
                    success_probability: 0.90,
                },
            ],
            learning_goals: vec![
                "Master Rust async email handling".to_string(),
                "Understand cron scheduling in Rust".to_string(),
                "Learn advanced log parsing techniques".to_string(),
            ],
            focus_time_blocks: vec![
                FocusBlock {
                    time_start: "09:00".to_string(),
                    time_end: "11:00".to_string(),
                    activity: "Deep coding session".to_string(),
                    context: "Email service completion".to_string(),
                },
                FocusBlock {
                    time_start: "14:00".to_string(),
                    time_end: "16:00".to_string(),
                    activity: "Testing and integration".to_string(),
                    context: "End-to-end system validation".to_string(),
                },
            ],
            optimization_suggestions: vec![
                "Use separate terminal sessions for different projects".to_string(),
                "Implement git hooks for automatic testing".to_string(),
                "Set up development environment templates".to_string(),
            ],
            estimated_complexity: 6.5,
        };

        let learning_recommendations = vec![
            "Document WARP COMMAND architecture for future reference".to_string(),
            "Create video walkthrough of system capabilities".to_string(),
            "Research advanced productivity tracking techniques".to_string(),
        ];

        let celebration_moments = vec![
            "🎉 Successfully built a sophisticated development intelligence system!".to_string(),
            "🚀 Processed 153,356+ log entries in first major analysis".to_string(),
            "🧠 Created self-improving AI system for development insights".to_string(),
        ];

        let technical_debt_alerts = if activity.productivity_metrics.error_rate > 0.1 {
            vec!["High error rate detected - consider reviewing recent changes".to_string()]
        } else {
            vec![]
        };

        Ok(DailyReport {
            report_date,
            yesterday_summary,
            today_plan,
            insights,
            learning_recommendations,
            productivity_score: activity.productivity_metrics.efficiency_score,
            focus_areas: activity.project_focus.iter()
                .map(|pf| format!("{} ({})", pf.project_name, pf.activity_type))
                .collect(),
            technical_debt_alerts,
            celebration_moments,
        })
    }
}
