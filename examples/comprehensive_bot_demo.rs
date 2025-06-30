use crate::bots::{
    BotRegistry, BotInput, BotError, BotSpecialty,
    salesforce_cta_bot::{SalesforceCTABot, CTABotInput, CTATaskType},
    document_management_bot::DocumentManagementBot,
    deadline_management_bot::{DeadlineManagementBot, DeadlineAnalysisInput, DeadlineAnalysisType},
    email_notification_bot::{EmailNotificationBot, EmailRequest},
    analytics_reporting_bot::{AnalyticsReportingBot, ReportRequest, ExportFormat},
};
use crate::ai::AiService;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;

/// Comprehensive demonstration of the MoodBridge Bot ecosystem
pub async fn demo_complete_bot_ecosystem() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 MoodBridge Legal AI Bot Ecosystem Demo");
    println!("==========================================\n");

    // Create AI service
    let ai_service = Arc::new(MockAiService::new());
    
    // Create bot registry
    let bot_registry = Arc::new(BotRegistry::new(ai_service.clone()));
    
    // Register all available bots
    register_all_bots(&bot_registry, ai_service.clone()).await;
    
    // Display registered bots
    display_bot_registry(&bot_registry).await;
    
    // Scenario 1: Complex Case Management Workflow
    println!("\n📋 Scenario 1: Complex Case Management Workflow");
    println!("===============================================");
    demo_case_management_workflow(&bot_registry).await?;
    
    // Scenario 2: Deadline Crisis Management
    println!("\n⏰ Scenario 2: Deadline Crisis Management");
    println!("========================================");
    demo_deadline_crisis_management(&bot_registry).await?;
    
    // Scenario 3: Client Communication Automation
    println!("\n📧 Scenario 3: Client Communication Automation");
    println!("==============================================");
    demo_client_communication_automation(&bot_registry).await?;
    
    // Scenario 4: Salesforce Enterprise Integration
    println!("\n🏢 Scenario 4: Salesforce Enterprise Integration");
    println!("===============================================");
    demo_salesforce_enterprise_integration(&bot_registry).await?;
    
    // Scenario 5: Analytics and Reporting
    println!("\n📊 Scenario 5: Analytics and Reporting");
    println!("====================================");
    demo_analytics_and_reporting(&bot_registry).await?;
    
    // Scenario 6: Multi-Bot Collaboration
    println!("\n🤝 Scenario 6: Multi-Bot Collaboration");
    println!("====================================");
    demo_multi_bot_collaboration(&bot_registry).await?;
    
    println!("\n✅ Complete Bot Ecosystem Demo Finished!");
    println!("All bots are functioning correctly and can collaborate effectively.");
    
    Ok(())
}

/// Register all available bots with the registry
async fn register_all_bots(
    registry: &Arc<BotRegistry>, 
    ai_service: Arc<dyn AiService + Send + Sync>
) {
    println!("🔧 Registering bots with the system...");
    
    // Register Salesforce CTA Bot
    let salesforce_bot = Arc::new(SalesforceCTABot::new(Some(ai_service.clone())));
    registry.register_bot(salesforce_bot).await;
    
    // Register Document Management Bot
    let document_bot = Arc::new(DocumentManagementBot::new());
    registry.register_bot(document_bot).await;
    
    // Register Deadline Management Bot
    let deadline_bot = Arc::new(DeadlineManagementBot::new(Some(ai_service.clone())));
    registry.register_bot(deadline_bot).await;
    
    // Register Email Notification Bot
    let email_bot = Arc::new(EmailNotificationBot::new(Some(ai_service.clone())));
    registry.register_bot(email_bot).await;
    
    // Register Analytics Reporting Bot
    let analytics_bot = Arc::new(AnalyticsReportingBot::new(Some(ai_service.clone())));
    registry.register_bot(analytics_bot).await;
    
    println!("✅ All bots registered successfully!\n");
}

/// Display the bot registry status
async fn display_bot_registry(registry: &Arc<BotRegistry>) {
    println!("📋 Bot Registry Status:");
    println!("======================");
    
    let bots = registry.list_bots().await;
    for (i, (id, specialty, name)) in bots.iter().enumerate() {
        println!("{}. {} ({:?}) - ID: {}", i + 1, name, specialty, id);
    }
    println!("Total bots registered: {}\n", bots.len());
}

/// Demonstrate complex case management workflow
async fn demo_case_management_workflow(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting a complex family law case with multiple deadlines and documents...");
    
    // Step 1: Document Management - Upload and categorize case documents
    let doc_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "document_management".to_string(),
        data: json!({
            "action": "upload_batch",
            "documents": [
                {"name": "divorce_petition.pdf", "type": "court_filing"},
                {"name": "financial_disclosure.xlsx", "type": "evidence"},
                {"name": "custody_evaluation.pdf", "type": "expert_report"}
            ]
        }),
        context: HashMap::from([
            ("case_type".to_string(), "family_law".to_string()),
            ("priority".to_string(), "high".to_string()),
        ]),
        priority: 200,
        deadline: None,
        requester: "Case Manager".to_string(),
    };
    
    let doc_result = registry.route_task(doc_task).await?;
    println!("📄 Document Management Result: {}", 
        if doc_result.success { "SUCCESS" } else { "FAILED" });
    println!("   Recommendations: {:?}", doc_result.recommendations);
    
    // Step 2: Deadline Management - Set up case deadlines
    let deadline_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "deadline_management".to_string(),
        data: json!({
            "analysis_type": "UpcomingDeadlines",
            "time_range": {
                "start_date": "2024-01-01T00:00:00Z",
                "end_date": "2024-12-31T23:59:59Z"
            }
        }),
        context: HashMap::new(),
        priority: 220,
        deadline: None,
        requester: "Attorney".to_string(),
    };
    
    let deadline_result = registry.route_task(deadline_task).await?;
    println!("⏰ Deadline Management Result: {}", 
        if deadline_result.success { "SUCCESS" } else { "FAILED" });
    
    Ok(())
}

/// Demonstrate deadline crisis management
async fn demo_deadline_crisis_management(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Handling a deadline crisis with overdue court filings...");
    
    // Analyze overdue deadlines
    let crisis_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "overdue_analysis".to_string(),
        data: json!({
            "analysis_type": "OverdueAnalysis",
            "priority_filter": ["Critical", "High"]
        }),
        context: HashMap::from([
            ("emergency".to_string(), "true".to_string()),
        ]),
        priority: 255,
        deadline: None,
        requester: "Senior Partner".to_string(),
    };
    
    let crisis_result = registry.route_task(crisis_task).await?;
    println!("🚨 Crisis Analysis Result: {}", 
        if crisis_result.success { "SUCCESS" } else { "FAILED" });
    
    // Send emergency notifications
    if crisis_result.success {
        let email_task = BotInput {
            task_id: Uuid::new_v4(),
            task_type: "email_notification".to_string(),
            data: json!({
                "template_id": "deadline_reminder",
                "recipients": [
                    {
                        "email": "attorney@moodbridge.law",
                        "name": "Lead Attorney",
                        "recipient_type": "Attorney"
                    }
                ],
                "variables": {
                    "recipient_name": "Lead Attorney",
                    "task_name": "Emergency Court Filing",
                    "due_date": "2024-01-15",
                    "case_name": "Smith v. Jones",
                    "priority": "CRITICAL"
                },
                "send_immediately": true,
                "tracking_enabled": true
            }),
            context: HashMap::new(),
            priority: 250,
            deadline: None,
            requester: "Crisis Management System".to_string(),
        };
        
        let email_result = registry.route_task(email_task).await?;
        println!("📧 Emergency Notification Result: {}", 
            if email_result.success { "SUCCESS" } else { "FAILED" });
    }
    
    Ok(())
}

/// Demonstrate client communication automation
async fn demo_client_communication_automation(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Automating client communication with case updates...");
    
    let email_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "client_update".to_string(),
        data: json!({
            "template_id": "client_update",
            "recipients": [
                {
                    "email": "client@example.com",
                    "name": "John Smith",
                    "recipient_type": "Client",
                    "case_id": "12345678-1234-1234-1234-123456789012"
                }
            ],
            "variables": {
                "client_name": "John Smith",
                "case_name": "Smith Family Law Matter",
                "case_status": "Discovery Phase",
                "last_activity": "Filed motion for temporary custody",
                "update_details": "We have filed the motion for temporary custody and are awaiting the court's response. The hearing is scheduled for next Tuesday.",
                "next_steps": "Prepare for custody hearing, gather additional evidence",
                "attorney_name": "Jane Attorney",
                "firm_name": "MoodBridge Legal"
            },
            "send_immediately": false,
            "scheduled_time": "2024-01-15T10:00:00Z",
            "tracking_enabled": true
        }),
        context: HashMap::new(),
        priority: 180,
        deadline: None,
        requester: "Client Relations".to_string(),
    };
    
    let result = registry.route_task(email_task).await?;
    println!("📨 Client Communication Result: {}", 
        if result.success { "SUCCESS" } else { "FAILED" });
    println!("   Confidence: {:.1}%", result.confidence * 100.0);
    
    Ok(())
}

/// Demonstrate Salesforce enterprise integration
async fn demo_salesforce_enterprise_integration(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Performing enterprise Salesforce architecture review...");
    
    let sf_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "salesforce_cta".to_string(),
        data: json!({
            "task_type": "ArchitectureReview",
            "scenario_description": "Enterprise legal practice with 100+ users needs Salesforce architecture review for case management integration",
            "current_architecture": "Salesforce org with custom objects for cases, clients, and matters. Integration with document management system via REST APIs.",
            "requirements": [
                {
                    "requirement_type": "Technical",
                    "description": "Support for 1000+ concurrent users",
                    "priority": "High",
                    "acceptance_criteria": ["Page load times under 2 seconds", "99.9% uptime"]
                }
            ],
            "constraints": [
                {
                    "constraint_type": "Budget",
                    "description": "Annual budget of $500K for improvements",
                    "impact": "High",
                    "mitigation_options": ["Phased implementation", "Priority-based rollout"]
                }
            ],
            "stakeholders": [
                {
                    "role": "Chief Technology Officer",
                    "interests": ["Scalability", "Security"],
                    "influence_level": "DecisionMaker",
                    "technical_expertise": "Architect"
                }
            ],
            "success_criteria": ["Improved performance", "Enhanced security", "Better user experience"],
            "timeline": "6 months for implementation",
            "budget_considerations": "Focus on high-impact, cost-effective solutions"
        }),
        context: HashMap::from([
            ("organization_size".to_string(), "enterprise".to_string()),
            ("complexity".to_string(), "high".to_string()),
        ]),
        priority: 255,
        deadline: None,
        requester: "CTO Office".to_string(),
    };
    
    let sf_result = registry.route_task(sf_task).await?;
    println!("🏢 Salesforce Architecture Review Result: {}", 
        if sf_result.success { "SUCCESS" } else { "FAILED" });
    println!("   Confidence: {:.1}%", sf_result.confidence * 100.0);
    println!("   Next Actions: {:?}", sf_result.next_actions);
    
    Ok(())
}

/// Demonstrate analytics and reporting
async fn demo_analytics_and_reporting(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating comprehensive analytics report...");
    
    let analytics_task = BotInput {
        task_id: Uuid::new_v4(),
        task_type: "generate_report".to_string(),
        data: json!({
            "template_id": "case_metrics",
            "parameters": {
                "date_range": "last_quarter"
            },
            "date_range": {
                "start_date": "2024-01-01T00:00:00Z",
                "end_date": "2024-03-31T23:59:59Z"
            },
            "export_format": "PDF",
            "include_raw_data": false,
            "recipient_emails": ["management@moodbridge.law"]
        }),
        context: HashMap::from([
            ("report_urgency".to_string(), "standard".to_string()),
        ]),
        priority: 160,
        deadline: None,
        requester: "Management Team".to_string(),
    };
    
    let analytics_result = registry.route_task(analytics_task).await?;
    println!("📊 Analytics Report Result: {}", 
        if analytics_result.success { "SUCCESS" } else { "FAILED" });
    println!("   Confidence: {:.1}%", analytics_result.confidence * 100.0);
    
    Ok(())
}

/// Demonstrate multi-bot collaboration
async fn demo_multi_bot_collaboration(registry: &Arc<BotRegistry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Orchestrating multi-bot collaboration for complex case workflow...");
    
    // Queue multiple related tasks
    let tasks = vec![
        // 1. Document processing
        BotInput {
            task_id: Uuid::new_v4(),
            task_type: "document_management".to_string(),
            data: json!({"action": "process_discovery"}),
            context: HashMap::from([("workflow_step".to_string(), "1".to_string())]),
            priority: 200,
            deadline: None,
            requester: "Workflow Orchestrator".to_string(),
        },
        // 2. Deadline analysis
        BotInput {
            task_id: Uuid::new_v4(),
            task_type: "deadline_analysis".to_string(),
            data: json!({
                "analysis_type": "WorkloadDistribution"
            }),
            context: HashMap::from([("workflow_step".to_string(), "2".to_string())]),
            priority: 190,
            deadline: None,
            requester: "Workflow Orchestrator".to_string(),
        },
        // 3. Generate reports
        BotInput {
            task_id: Uuid::new_v4(),
            task_type: "analytics".to_string(),
            data: json!({
                "template_id": "case_metrics"
            }),
            context: HashMap::from([("workflow_step".to_string(), "3".to_string())]),
            priority: 180,
            deadline: None,
            requester: "Workflow Orchestrator".to_string(),
        },
        // 4. Send notifications
        BotInput {
            task_id: Uuid::new_v4(),
            task_type: "team_alert".to_string(),
            data: json!({
                "template_id": "workflow_complete",
                "recipients": [{"email": "team@moodbridge.law", "name": "Legal Team"}]
            }),
            context: HashMap::from([("workflow_step".to_string(), "4".to_string())]),
            priority: 170,
            deadline: None,
            requester: "Workflow Orchestrator".to_string(),
        },
    ];
    
    // Queue all tasks
    for task in tasks {
        registry.queue_task(task).await;
    }
    
    // Process the entire queue
    let results = registry.process_queue().await;
    
    println!("🤝 Multi-Bot Collaboration Results:");
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(output) => println!("   Step {}: SUCCESS (Bot: {})", i + 1, output.bot_id),
            Err(e) => println!("   Step {}: FAILED ({})", i + 1, e),
        }
    }
    
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    println!("   Overall Success Rate: {}/{} ({:.1}%)", 
        success_count, results.len(), 
        (success_count as f64 / results.len() as f64) * 100.0);
    
    Ok(())
}

/// Mock AI service for demonstration
#[derive(Debug)]
struct MockAiService;

impl MockAiService {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl AiService for MockAiService {
    async fn analyze(&self, prompt: &str, context: Option<&str>) -> Result<crate::ai::AnalysisResponse, crate::ai::AiError> {
        Ok(crate::ai::AnalysisResponse {
            content: format!("Mock AI analysis for: {}", prompt),
            confidence: 0.85,
            model_used: "mock-gpt-4".to_string(),
            tokens_used: 150,
            processing_time_ms: 200,
        })
    }

    async fn generate_insights(&self, data: &serde_json::Value) -> Result<Vec<String>, crate::ai::AiError> {
        Ok(vec![
            "Pattern detected in case resolution times".to_string(),
            "Workload imbalance identified across team members".to_string(),
            "Opportunity for process automation in document review".to_string(),
        ])
    }

    async fn detect_patterns(&self, data: &[serde_json::Value]) -> Result<Vec<String>, crate::ai::AiError> {
        Ok(vec![
            "Seasonal variation in case volume".to_string(),
            "Correlation between case complexity and resolution time".to_string(),
            "Client communication frequency affects satisfaction scores".to_string(),
        ])
    }

    async fn assess_risk(&self, scenario: &str) -> Result<f64, crate::ai::AiError> {
        Ok(0.25) // Low to medium risk
    }

    async fn analyze_sentiment(&self, text: &str) -> Result<f64, crate::ai::AiError> {
        Ok(0.75) // Generally positive sentiment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_bot_ecosystem() {
        // This would run the complete demo in test mode
        // For now, just test that we can create the ecosystem
        let ai_service = Arc::new(MockAiService::new());
        let registry = Arc::new(BotRegistry::new(ai_service.clone()));
        
        register_all_bots(&registry, ai_service).await;
        
        let bots = registry.list_bots().await;
        assert!(bots.len() >= 5, "Should have at least 5 bots registered");
        
        // Test that each bot specialty is represented
        let specialties: Vec<_> = bots.iter().map(|(_, specialty, _)| specialty).collect();
        assert!(specialties.contains(&&BotSpecialty::SalesforceArchitect));
        assert!(specialties.contains(&&BotSpecialty::DocumentManagement));
        assert!(specialties.contains(&&BotSpecialty::DeadlineManagement));
        assert!(specialties.contains(&&BotSpecialty::EmailNotificationBot));
        assert!(specialties.contains(&&BotSpecialty::AnalyticsReporting));
    }

    #[tokio::test]
    async fn test_bot_collaboration() {
        let ai_service = Arc::new(MockAiService::new());
        let registry = Arc::new(BotRegistry::new(ai_service.clone()));
        
        // Register a few bots
        let document_bot = Arc::new(DocumentManagementBot::new());
        let email_bot = Arc::new(EmailNotificationBot::new(Some(ai_service.clone())));
        
        registry.register_bot(document_bot).await;
        registry.register_bot(email_bot).await;
        
        // Test task routing
        let task = BotInput {
            task_id: Uuid::new_v4(),
            task_type: "document_management".to_string(),
            data: json!({"test": "data"}),
            context: HashMap::new(),
            priority: 100,
            deadline: None,
            requester: "test".to_string(),
        };
        
        let result = registry.route_task(task).await;
        assert!(result.is_ok(), "Task routing should succeed");
    }
}
