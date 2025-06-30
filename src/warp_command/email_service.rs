use super::*;
use lettre::{
    transport::smtp::authentication::Credentials,
    message::header::ContentType,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor
};

pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    sender_email: String,
    recipient_email: String,
}

impl EmailService {
    pub fn new(config: &WarpCommandConfig) -> Result<Self, WarpCommandError> {
        let credentials = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );

        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .map_err(|e| WarpCommandError::EmailError(format!("SMTP relay error: {}", e)))?
            .port(config.smtp_port)
            .credentials(credentials)
            .build();

        Ok(Self {
            smtp_transport,
            sender_email: config.email_sender.clone(),
            recipient_email: config.email_recipient.clone(),
        })
    }

    /// Send a test email to verify configuration
    pub async fn send_test_email(&self) -> Result<(), WarpCommandError> {
        let subject = "🚀 WARP COMMAND Test - System Operational";
        let body = self.create_test_email_body();

        self.send_email(subject, &body).await
    }

    /// Send the daily WARP COMMAND report
    pub async fn send_daily_report(&self, report: &DailyReport) -> Result<(), WarpCommandError> {
        let subject = format!("📊 WARP COMMAND Daily Report - {}", 
            report.report_date.format("%Y-%m-%d"));
        let body = self.create_daily_report_body(report);

        self.send_email(&subject, &body).await
    }

    async fn send_email(&self, subject: &str, body: &str) -> Result<(), WarpCommandError> {
        let email = Message::builder()
            .from(self.sender_email.parse()
                .map_err(|e| WarpCommandError::EmailError(format!("Invalid sender email: {}", e)))?)
            .to(self.recipient_email.parse()
                .map_err(|e| WarpCommandError::EmailError(format!("Invalid recipient email: {}", e)))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| WarpCommandError::EmailError(format!("Failed to build email: {}", e)))?;

        // use lettre::Transport;
        self.smtp_transport
            .send(email)
            .await
            .map_err(|e| WarpCommandError::EmailError(format!("Failed to send email: {}", e)))?;

        tracing::info!("Email sent successfully to {}", self.recipient_email);
        Ok(())
    }

    fn create_test_email_body(&self) -> String {
        let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }}
        .container {{ max-width: 600px; margin: 0 auto; background: white; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 12px 12px 0 0; text-align: center; }}
        .content {{ padding: 30px; }}
        .status {{ background: #10b981; color: white; padding: 15px; border-radius: 8px; text-align: center; margin: 20px 0; }}
        .feature {{ background: #f8fafc; padding: 20px; border-radius: 8px; margin: 15px 0; border-left: 4px solid #667eea; }}
        .footer {{ background: #f8fafc; padding: 20px; border-radius: 0 0 12px 12px; text-align: center; color: #64748b; }}
        h1 {{ margin: 0; font-size: 28px; }}
        h2 {{ color: #334155; margin-top: 0; }}
        .emoji {{ font-size: 24px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="emoji">🚀</div>
            <h1>WARP COMMAND</h1>
            <p>AI-Powered Development Intelligence System</p>
        </div>
        
        <div class="content">
            <div class="status">
                <strong>✅ System Test Successful</strong>
            </div>
            
            <h2>🎯 Test Completed</h2>
            <p>Your WARP COMMAND system is now operational and ready to analyze your development patterns!</p>
            
            <div class="feature">
                <h2>📊 What WARP COMMAND Will Do</h2>
                <ul>
                    <li><strong>Daily Log Analysis:</strong> Parse your Warp terminal logs every hour</li>
                    <li><strong>Development Insights:</strong> Track productivity, learning patterns, and code focus</li>
                    <li><strong>Smart Recommendations:</strong> AI-powered suggestions for improving workflow</li>
                    <li><strong>Morning Reports:</strong> Daily emails at 7am Central with actionable insights</li>
                </ul>
            </div>
            
            <div class="feature">
                <h2>🧠 Self-Improvement Engine</h2>
                <p>The system learns from your patterns and continuously improves its recommendations. It will:</p>
                <ul>
                    <li>Identify your most productive times and environments</li>
                    <li>Detect learning opportunities and skill development areas</li>
                    <li>Suggest optimizations based on your unique workflow</li>
                    <li>Celebrate wins and highlight progress</li>
                </ul>
            </div>
            
            <div class="feature">
                <h2>📈 Today's Build Day Analysis</h2>
                <p>With <strong>153,356 log entries</strong> from today, WARP COMMAND will provide rich insights about your significant build day on the MoodBridge Rust project.</p>
            </div>
        </div>
        
        <div class="footer">
            <p>Test sent at: {}</p>
            <p>🦀 Built with Rust • ⚡ Powered by AI • 📧 Delivered to tbcolby@pm.me</p>
        </div>
    </div>
</body>
</html>
"#, current_time)
    }

    fn create_daily_report_body(&self, report: &DailyReport) -> String {
        let date = report.report_date.format("%A, %B %d, %Y");
        
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }}
        .container {{ max-width: 700px; margin: 0 auto; background: white; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 12px 12px 0 0; text-align: center; }}
        .content {{ padding: 30px; }}
        .section {{ margin: 25px 0; }}
        .metric {{ background: #f8fafc; padding: 20px; border-radius: 8px; margin: 15px 0; border-left: 4px solid #10b981; }}
        .insight {{ background: #fef3c7; padding: 15px; border-radius: 8px; margin: 10px 0; border-left: 4px solid #f59e0b; }}
        .task {{ background: #e0f2fe; padding: 15px; border-radius: 8px; margin: 10px 0; border-left: 4px solid #0284c7; }}
        .celebration {{ background: #f0fdf4; padding: 15px; border-radius: 8px; margin: 10px 0; border-left: 4px solid #22c55e; }}
        .footer {{ background: #f8fafc; padding: 20px; border-radius: 0 0 12px 12px; text-align: center; color: #64748b; }}
        h1 {{ margin: 0; font-size: 28px; }}
        h2 {{ color: #334155; border-bottom: 2px solid #e2e8f0; padding-bottom: 10px; }}
        .score {{ font-size: 24px; font-weight: bold; color: #10b981; }}
        .priority-high {{ border-left-color: #ef4444; }}
        .priority-medium {{ border-left-color: #f59e0b; }}
        .priority-low {{ border-left-color: #10b981; }}
        ul {{ margin: 10px 0; }}
        li {{ margin: 5px 0; }}
        .emoji {{ font-size: 20px; margin-right: 8px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 WARP COMMAND</h1>
            <p>Daily Development Intelligence Report</p>
            <p>{}</p>
        </div>
        
        <div class="content">
            <div class="section">
                <h2>🎯 Productivity Score</h2>
                <div class="metric">
                    <span class="score">{:.1}/10.0</span>
                    <p>{}</p>
                </div>
            </div>

            <div class="section">
                <h2>🏆 Yesterday's Accomplishments</h2>
                {}
            </div>

            <div class="section">
                <h2>🚀 Today's Priority Tasks</h2>
                {}
            </div>

            <div class="section">
                <h2>💡 AI Insights & Recommendations</h2>
                {}
            </div>

            <div class="section">
                <h2>🎓 Learning & Growth</h2>
                {}
            </div>

            <div class="section">
                <h2>🎉 Celebration Moments</h2>
                {}
            </div>

            <div class="section">
                <h2>⚠️ Technical Debt Alerts</h2>
                {}
            </div>
        </div>
        
        <div class="footer">
            <p>🤖 Generated by WARP COMMAND AI • Next report: Tomorrow 7:00 AM CT</p>
            <p>System learning and improving from your development patterns</p>
        </div>
    </div>
</body>
</html>
"#, 
            date,
            report.productivity_score,
            self.get_productivity_message(report.productivity_score),
            self.format_accomplishments(&report.yesterday_summary.accomplishments),
            self.format_priority_tasks(&report.today_plan.priority_tasks),
            self.format_insights(&report.insights),
            self.format_learning_recommendations(&report.learning_recommendations),
            self.format_celebrations(&report.celebration_moments),
            self.format_technical_debt(&report.technical_debt_alerts)
        )
    }

    fn get_productivity_message(&self, score: f64) -> &str {
        match score {
            s if s >= 8.0 => "🔥 Outstanding productivity day! You're in the zone!",
            s if s >= 6.0 => "💪 Strong productive session with solid progress",
            s if s >= 4.0 => "👍 Good steady progress, building momentum",
            _ => "🌱 Every step forward counts - keep building!"
        }
    }

    fn format_accomplishments(&self, accomplishments: &[String]) -> String {
        if accomplishments.is_empty() {
            return "<p><em>Starting fresh today! Yesterday's work is building toward today's progress.</em></p>".to_string();
        }
        
        accomplishments.iter()
            .map(|acc| format!("<div class=\"celebration\"><span class=\"emoji\">✅</span>{}</div>", acc))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_priority_tasks(&self, tasks: &[PriorityTask]) -> String {
        if tasks.is_empty() {
            return "<p><em>No specific tasks identified - focus on continuing your development momentum!</em></p>".to_string();
        }
        
        tasks.iter()
            .map(|task| {
                let complexity_emoji = match task.complexity {
                    c if c >= 7.0 => "🔥",
                    c if c >= 4.0 => "⚡",
                    _ => "🎯"
                };
                format!(
                    "<div class=\"task\"><span class=\"emoji\">{}</span><strong>{}</strong><br>
                    <small>Est: {} min • Complexity: {:.1}/10 • Success: {:.0}%</small></div>", 
                    complexity_emoji, task.task, task.estimated_time_minutes, task.complexity, task.success_probability * 100.0
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_insights(&self, insights: &[ActivityInsight]) -> String {
        if insights.is_empty() {
            return "<p><em>Building insights as patterns emerge from your development activity...</em></p>".to_string();
        }
        
        insights.iter()
            .map(|insight| {
                let priority_class = match insight.priority {
                    Priority::High => "priority-high",
                    Priority::Medium => "priority-medium",
                    Priority::Low => "priority-low",
                    Priority::Critical => "priority-high"
                };
                let emoji = match insight.insight_type {
                    InsightType::ProductivityTrend => "📈",
                    InsightType::LearningPattern => "🧠",
                    InsightType::WorkflowEfficiency => "⚡",
                    _ => "💡"
                };
                format!(
                    "<div class=\"insight {}\"><span class=\"emoji\">{}</span><strong>{}</strong><br>
                    <ul>{}</ul></div>", 
                    priority_class,
                    emoji,
                    insight.message,
                    insight.recommendations.iter()
                        .map(|rec| format!("<li>{}</li>", rec))
                        .collect::<Vec<_>>()
                        .join("")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_learning_recommendations(&self, recommendations: &[String]) -> String {
        if recommendations.is_empty() {
            return "<p><em>Keep building - learning opportunities will be identified as patterns develop!</em></p>".to_string();
        }
        
        recommendations.iter()
            .map(|rec| format!("<div class=\"insight\"><span class=\"emoji\">📚</span>{}</div>", rec))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_celebrations(&self, celebrations: &[String]) -> String {
        if celebrations.is_empty() {
            return "<p><em>Every coding session is progress worth celebrating! 🎉</em></p>".to_string();
        }
        
        celebrations.iter()
            .map(|cel| format!("<div class=\"celebration\"><span class=\"emoji\">🎉</span>{}</div>", cel))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_technical_debt(&self, alerts: &[String]) -> String {
        if alerts.is_empty() {
            return "<p><em>✅ No immediate technical debt concerns detected</em></p>".to_string();
        }
        
        alerts.iter()
            .map(|alert| format!("<div class=\"insight priority-high\"><span class=\"emoji\">⚠️</span>{}</div>", alert))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
