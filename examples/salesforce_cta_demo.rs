use crate::bots::{
    BotRegistry, BotInput, BotError,
    salesforce_cta_bot::{
        SalesforceCTABot, CTABotInput, CTATaskType, Requirement, RequirementType, 
        Priority, Constraint, ConstraintType, ImpactLevel, Stakeholder, 
        InfluenceLevel, TechnicalExpertise
    }
};
use crate::ai::AiService;
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::json;

/// Example usage of the Salesforce CTA Bot
pub async fn demo_salesforce_cta_bot() -> Result<(), Box<dyn std::error::Error>> {
    // Create AI service (mock for this example)
    let ai_service = Arc::new(MockAiService::new());
    
    // Create bot registry
    let bot_registry = BotRegistry::new(ai_service.clone());
    
    // Create and register the Salesforce CTA Bot
    let cta_bot = Arc::new(SalesforceCTABot::new(Some(ai_service.clone())));
    bot_registry.register_bot(cta_bot.clone()).await;
    
    // Example 1: Architecture Review for Enterprise Salesforce Implementation
    println!("=== Architecture Review Demo ===");
    let architecture_review_task = create_architecture_review_task()?;
    let result = bot_registry.route_task(architecture_review_task).await?;
    println!("Architecture Review Result: {}", serde_json::to_string_pretty(&result)?);
    
    // Example 2: Solution Design for Multi-Cloud Integration
    println!("\n=== Solution Design Demo ===");
    let solution_design_task = create_solution_design_task()?;
    let result = bot_registry.route_task(solution_design_task).await?;
    println!("Solution Design Result: {}", serde_json::to_string_pretty(&result)?);
    
    // Example 3: Security Assessment for GDPR Compliance
    println!("\n=== Security Assessment Demo ===");
    let security_assessment_task = create_security_assessment_task()?;
    let result = bot_registry.route_task(security_assessment_task).await?;
    println!("Security Assessment Result: {}", serde_json::to_string_pretty(&result)?);
    
    // Example 4: Performance Optimization for High-Volume Data Processing
    println!("\n=== Performance Optimization Demo ===");
    let performance_task = create_performance_optimization_task()?;
    let result = bot_registry.route_task(performance_task).await?;
    println!("Performance Optimization Result: {}", serde_json::to_string_pretty(&result)?);
    
    Ok(())
}

/// Create an architecture review task for a large enterprise Salesforce implementation
fn create_architecture_review_task() -> Result<BotInput, Box<dyn std::error::Error>> {
    let cta_input = CTABotInput {
        task_type: CTATaskType::ArchitectureReview,
        scenario_description: "Large multinational corporation with 50,000+ users across 30 countries needs comprehensive architecture review of their Salesforce org before major expansion".to_string(),
        current_architecture: Some("Current setup: Sales Cloud, Service Cloud, Marketing Cloud, with custom Lightning components, 200+ integrations via REST APIs, on-premise ERP integration via MuleSoft, multiple sandboxes".to_string()),
        requirements: vec![
            Requirement {
                requirement_type: RequirementType::NonFunctional,
                description: "Support 99.9% uptime with sub-2 second page load times".to_string(),
                priority: Priority::Critical,
                acceptance_criteria: vec![
                    "Page load times consistently under 2 seconds".to_string(),
                    "System availability of 99.9% measured monthly".to_string(),
                    "Zero data loss during maintenance windows".to_string(),
                ],
            },
            Requirement {
                requirement_type: RequirementType::Security,
                description: "Implement enterprise-grade security with GDPR and SOX compliance".to_string(),
                priority: Priority::High,
                acceptance_criteria: vec![
                    "All PII data encrypted at rest and in transit".to_string(),
                    "Audit trails for all data access and modifications".to_string(),
                    "Role-based access control with principle of least privilege".to_string(),
                ],
            },
            Requirement {
                requirement_type: RequirementType::Technical,
                description: "Support real-time integration with 50+ external systems".to_string(),
                priority: Priority::High,
                acceptance_criteria: vec![
                    "Real-time data synchronization with ERP systems".to_string(),
                    "API rate limits managed effectively".to_string(),
                    "Error handling and retry mechanisms in place".to_string(),
                ],
            },
        ],
        constraints: vec![
            Constraint {
                constraint_type: ConstraintType::Budget,
                description: "Annual budget cap of $2M for platform improvements".to_string(),
                impact: ImpactLevel::High,
                mitigation_options: vec![
                    "Phased implementation over 18 months".to_string(),
                    "Prioritize high-impact, low-cost improvements".to_string(),
                    "Leverage existing Salesforce licenses".to_string(),
                ],
            },
            Constraint {
                constraint_type: ConstraintType::Timeline,
                description: "Major improvements must be completed before Q4 2024".to_string(),
                impact: ImpactLevel::Medium,
                mitigation_options: vec![
                    "Parallel development tracks".to_string(),
                    "Automated testing and deployment".to_string(),
                    "Dedicated project team".to_string(),
                ],
            },
        ],
        stakeholders: vec![
            Stakeholder {
                role: "Chief Technology Officer".to_string(),
                interests: vec!["Platform stability".to_string(), "Cost optimization".to_string()],
                influence_level: InfluenceLevel::DecisionMaker,
                technical_expertise: TechnicalExpertise::Architect,
            },
            Stakeholder {
                role: "VP of Sales Operations".to_string(),
                interests: vec!["User experience".to_string(), "Sales productivity".to_string()],
                influence_level: InfluenceLevel::High,
                technical_expertise: TechnicalExpertise::PowerUser,
            },
            Stakeholder {
                role: "Salesforce System Administrator".to_string(),
                interests: vec!["Maintainability".to_string(), "User adoption".to_string()],
                influence_level: InfluenceLevel::Medium,
                technical_expertise: TechnicalExpertise::SystemAdmin,
            },
        ],
        success_criteria: vec![
            "Improved system performance metrics".to_string(),
            "Reduced maintenance overhead".to_string(),
            "Enhanced security posture".to_string(),
            "Better user satisfaction scores".to_string(),
        ],
        timeline: Some("12 months for complete implementation".to_string()),
        budget_considerations: Some("$2M annual budget with preference for OPEX over CAPEX".to_string()),
    };

    Ok(BotInput {
        task_id: Uuid::new_v4(),
        task_type: "architecture_review".to_string(),
        data: serde_json::to_value(cta_input)?,
        context: HashMap::new(),
        priority: 200,
        deadline: None,
        requester: "Enterprise Architecture Team".to_string(),
    })
}

/// Create a solution design task for multi-cloud integration
fn create_solution_design_task() -> Result<BotInput, Box<dyn std::error::Error>> {
    let cta_input = CTABotInput {
        task_type: CTATaskType::SolutionDesign,
        scenario_description: "Design a comprehensive solution for integrating Salesforce with AWS services, Microsoft Azure, and Google Cloud Platform for a hybrid cloud architecture".to_string(),
        current_architecture: Some("Salesforce org with basic integrations, AWS infrastructure with EC2 and S3, Azure AD for identity management, some GCP services for analytics".to_string()),
        requirements: vec![
            Requirement {
                requirement_type: RequirementType::Functional,
                description: "Real-time data synchronization across all cloud platforms".to_string(),
                priority: Priority::High,
                acceptance_criteria: vec![
                    "Data consistency across platforms within 30 seconds".to_string(),
                    "Automatic conflict resolution mechanisms".to_string(),
                    "Comprehensive audit logging".to_string(),
                ],
            },
            Requirement {
                requirement_type: RequirementType::Technical,
                description: "Unified identity and access management across all platforms".to_string(),
                priority: Priority::Critical,
                acceptance_criteria: vec![
                    "Single sign-on (SSO) across all platforms".to_string(),
                    "Centralized user provisioning and deprovisioning".to_string(),
                    "Multi-factor authentication enforcement".to_string(),
                ],
            },
        ],
        constraints: vec![
            Constraint {
                constraint_type: ConstraintType::Technical,
                description: "Must maintain existing integrations during migration".to_string(),
                impact: ImpactLevel::High,
                mitigation_options: vec![
                    "Blue-green deployment strategy".to_string(),
                    "Gradual migration with rollback capabilities".to_string(),
                    "Comprehensive testing in staging environments".to_string(),
                ],
            },
        ],
        stakeholders: vec![
            Stakeholder {
                role: "Cloud Architect".to_string(),
                interests: vec!["Technical excellence".to_string(), "Scalability".to_string()],
                influence_level: InfluenceLevel::High,
                technical_expertise: TechnicalExpertise::Architect,
            },
        ],
        success_criteria: vec![
            "Seamless multi-cloud integration".to_string(),
            "Improved data analytics capabilities".to_string(),
            "Enhanced disaster recovery".to_string(),
        ],
        timeline: Some("8 months for full implementation".to_string()),
        budget_considerations: Some("Focus on operational efficiency and cost optimization".to_string()),
    };

    Ok(BotInput {
        task_id: Uuid::new_v4(),
        task_type: "solution_design".to_string(),
        data: serde_json::to_value(cta_input)?,
        context: HashMap::new(),
        priority: 190,
        deadline: None,
        requester: "Cloud Strategy Team".to_string(),
    })
}

/// Create a security assessment task for GDPR compliance
fn create_security_assessment_task() -> Result<BotInput, Box<dyn std::error::Error>> {
    let cta_input = CTABotInput {
        task_type: CTATaskType::SecurityAssessment,
        scenario_description: "Comprehensive security assessment of Salesforce org to ensure GDPR compliance and implement advanced security measures for a financial services company".to_string(),
        current_architecture: Some("Salesforce Financial Services Cloud with custom objects for client data, integrated with core banking systems, mobile app for customer access".to_string()),
        requirements: vec![
            Requirement {
                requirement_type: RequirementType::Compliance,
                description: "Full GDPR compliance including right to be forgotten".to_string(),
                priority: Priority::Critical,
                acceptance_criteria: vec![
                    "Automated data subject request processing".to_string(),
                    "Complete data lineage tracking".to_string(),
                    "Consent management integration".to_string(),
                ],
            },
            Requirement {
                requirement_type: RequirementType::Security,
                description: "Implement zero-trust security model".to_string(),
                priority: Priority::High,
                acceptance_criteria: vec![
                    "Multi-factor authentication for all users".to_string(),
                    "Device-based access controls".to_string(),
                    "Continuous security monitoring".to_string(),
                ],
            },
        ],
        constraints: vec![
            Constraint {
                constraint_type: ConstraintType::Regulatory,
                description: "Must comply with financial services regulations".to_string(),
                impact: ImpactLevel::Critical,
                mitigation_options: vec![
                    "Regular compliance audits".to_string(),
                    "Automated compliance reporting".to_string(),
                    "Legal review of all changes".to_string(),
                ],
            },
        ],
        stakeholders: vec![
            Stakeholder {
                role: "Chief Information Security Officer".to_string(),
                interests: vec!["Security posture".to_string(), "Compliance adherence".to_string()],
                influence_level: InfluenceLevel::DecisionMaker,
                technical_expertise: TechnicalExpertise::Architect,
            },
            Stakeholder {
                role: "Data Protection Officer".to_string(),
                interests: vec!["Privacy compliance".to_string(), "Data governance".to_string()],
                influence_level: InfluenceLevel::High,
                technical_expertise: TechnicalExpertise::PowerUser,
            },
        ],
        success_criteria: vec![
            "Zero security incidents".to_string(),
            "100% GDPR compliance".to_string(),
            "Improved security awareness".to_string(),
        ],
        timeline: Some("6 months for implementation".to_string()),
        budget_considerations: Some("Security investment justified by risk reduction".to_string()),
    };

    Ok(BotInput {
        task_id: Uuid::new_v4(),
        task_type: "security_assessment".to_string(),
        data: serde_json::to_value(cta_input)?,
        context: HashMap::new(),
        priority: 220,
        deadline: None,
        requester: "Security Team".to_string(),
    })
}

/// Create a performance optimization task
fn create_performance_optimization_task() -> Result<BotInput, Box<dyn std::error::Error>> {
    let cta_input = CTABotInput {
        task_type: CTATaskType::PerformanceOptimization,
        scenario_description: "Optimize Salesforce org performance for high-volume data processing with 10M+ records and 1000+ concurrent users".to_string(),
        current_architecture: Some("Large Salesforce org with custom objects, complex workflows, batch processing, real-time integrations".to_string()),
        requirements: vec![
            Requirement {
                requirement_type: RequirementType::NonFunctional,
                description: "Handle 1000+ concurrent users with consistent performance".to_string(),
                priority: Priority::Critical,
                acceptance_criteria: vec![
                    "Page load times under 3 seconds for 95% of requests".to_string(),
                    "API response times under 500ms".to_string(),
                    "Batch processing completion within SLA windows".to_string(),
                ],
            },
        ],
        constraints: vec![
            Constraint {
                constraint_type: ConstraintType::Technical,
                description: "Cannot modify core business logic during optimization".to_string(),
                impact: ImpactLevel::Medium,
                mitigation_options: vec![
                    "Focus on database and query optimization".to_string(),
                    "Implement caching strategies".to_string(),
                    "Optimize integration patterns".to_string(),
                ],
            },
        ],
        stakeholders: vec![
            Stakeholder {
                role: "Performance Engineer".to_string(),
                interests: vec!["System performance".to_string(), "Scalability".to_string()],
                influence_level: InfluenceLevel::High,
                technical_expertise: TechnicalExpertise::Developer,
            },
        ],
        success_criteria: vec![
            "50% improvement in page load times".to_string(),
            "Reduced API timeout errors".to_string(),
            "Improved user satisfaction".to_string(),
        ],
        timeline: Some("4 months for optimization project".to_string()),
        budget_considerations: Some("Cost-effective solutions preferred".to_string()),
    };

    Ok(BotInput {
        task_id: Uuid::new_v4(),
        task_type: "performance_optimization".to_string(),
        data: serde_json::to_value(cta_input)?,
        context: HashMap::new(),
        priority: 180,
        deadline: None,
        requester: "Performance Team".to_string(),
    })
}

/// Mock AI service for demonstration purposes
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
        // Mock implementation - in real use this would call actual AI service
        Ok(crate::ai::AnalysisResponse {
            content: "Mock AI analysis response".to_string(),
            confidence: 0.85,
            model_used: "mock-model".to_string(),
            tokens_used: 100,
            processing_time_ms: 250,
        })
    }

    async fn generate_insights(&self, data: &serde_json::Value) -> Result<Vec<String>, crate::ai::AiError> {
        Ok(vec![
            "Identified potential performance bottleneck in data loading".to_string(),
            "Security vulnerability detected in API authentication".to_string(),
            "Opportunity for automation in manual processes".to_string(),
        ])
    }

    async fn detect_patterns(&self, data: &[serde_json::Value]) -> Result<Vec<String>, crate::ai::AiError> {
        Ok(vec![
            "Recurring integration failures during peak hours".to_string(),
            "User adoption patterns showing resistance to new features".to_string(),
            "Data quality issues in specific geographic regions".to_string(),
        ])
    }

    async fn assess_risk(&self, scenario: &str) -> Result<f64, crate::ai::AiError> {
        // Mock risk assessment - return a risk score between 0.0 and 1.0
        Ok(0.3) // Low to medium risk
    }

    async fn analyze_sentiment(&self, text: &str) -> Result<f64, crate::ai::AiError> {
        // Mock sentiment analysis
        Ok(0.7) // Positive sentiment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_salesforce_cta_bot_creation() {
        let ai_service = Arc::new(MockAiService::new());
        let bot = SalesforceCTABot::new(Some(ai_service));
        
        assert_eq!(bot.get_name(), "Salesforce CTA Bot");
        assert!(bot.can_handle("architecture_review").await);
        assert!(bot.can_handle("solution_design").await);
        assert!(bot.can_handle("security_assessment").await);
        assert!(!bot.can_handle("unrelated_task").await);
    }

    #[tokio::test]
    async fn test_architecture_review_task() {
        let task = create_architecture_review_task().unwrap();
        assert_eq!(task.task_type, "architecture_review");
        assert_eq!(task.priority, 200);
    }

    #[tokio::test]
    async fn test_bot_priority_levels() {
        let ai_service = Arc::new(MockAiService::new());
        let bot = SalesforceCTABot::new(Some(ai_service));
        
        assert_eq!(bot.get_priority("salesforce_cta"), 255);
        assert_eq!(bot.get_priority("architecture_review"), 200);
        assert_eq!(bot.get_priority("security_assessment"), 180);
        assert_eq!(bot.get_priority("unknown_task"), 100);
    }
}
