use super::*;
use crate::bots::BotSpecialty;
use crate::wizard::WizardType;
use chrono::Utc;
use serde_json::json;

/// Create the Salesforce CTA Portal configuration
pub fn create_salesforce_cta_portal() -> BotPortal {
    BotPortal {
        portal_id: "salesforce_cta".to_string(),
        bot_specialty: BotSpecialty::SalesforceArchitect,
        name: "Salesforce CTA Portal".to_string(),
        description: "Enterprise-grade Salesforce architecture guidance and technical consulting. Get expert recommendations for complex Salesforce implementations, integrations, and optimizations.".to_string(),
        icon: "⚡".to_string(),
        color_scheme: ColorScheme {
            primary: "#0176d3".to_string(),    // Salesforce Blue
            secondary: "#032d60".to_string(),   // Dark Blue
            accent: "#ffa500".to_string(),      // Orange
            background: "#f3f2f2".to_string(),  // Light Gray
            text: "#080707".to_string(),        // Dark Gray
            success: "#04844b".to_string(),     // Green
            warning: "#ffb75d".to_string(),     // Yellow
            error: "#c23934".to_string(),       // Red
        },
        status: PortalStatus::Active,
        setup_wizard: create_salesforce_setup_wizard(),
        dashboard_config: create_salesforce_dashboard(),
        quick_actions: create_salesforce_quick_actions(),
        navigation_menu: create_salesforce_navigation(),
        help_resources: create_salesforce_help_resources(),
        created_at: Utc::now(),
        last_accessed: None,
    }
}

fn create_salesforce_setup_wizard() -> SetupWizardConfig {
    SetupWizardConfig {
        wizard_id: "salesforce_cta_setup".to_string(),
        wizard_type: WizardType::SalesforceIntegration,
        required_steps: vec![
            WizardStepConfig {
                step_id: "sf_org_connection".to_string(),
                title: "Connect to Salesforce Org".to_string(),
                description: "Establish secure connection to your Salesforce organization".to_string(),
                step_type: WizardStepType::Authentication,
                required: true,
                estimated_minutes: 5,
                help_text: Some("You'll need System Administrator access to complete this step".to_string()),
            },
            WizardStepConfig {
                step_id: "sf_architecture_analysis".to_string(),
                title: "Architecture Analysis Setup".to_string(),
                description: "Configure analysis parameters for your Salesforce environment".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 8,
                help_text: Some("This helps us understand your current architecture and requirements".to_string()),
            },
            WizardStepConfig {
                step_id: "sf_integration_mapping".to_string(),
                title: "Integration Mapping".to_string(),
                description: "Map existing integrations and identify new integration points".to_string(),
                step_type: WizardStepType::Integration,
                required: false,
                estimated_minutes: 10,
                help_text: Some("Skip this step if you don't have existing integrations".to_string()),
            },
            WizardStepConfig {
                step_id: "sf_governance_setup".to_string(),
                title: "Governance Framework".to_string(),
                description: "Establish governance rules and compliance requirements".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 7,
                help_text: Some("Critical for enterprise deployments and regulatory compliance".to_string()),
            },
            WizardStepConfig {
                step_id: "sf_testing_validation".to_string(),
                title: "Testing & Validation".to_string(),
                description: "Run initial tests to validate the configuration".to_string(),
                step_type: WizardStepType::Testing,
                required: true,
                estimated_minutes: 5,
                help_text: Some("We'll test the connection and basic functionality".to_string()),
            },
        ],
        estimated_time_minutes: 35,
        prerequisites: vec![
            "Salesforce System Administrator access".to_string(),
            "Connected App configuration permissions".to_string(),
            "API access enabled in your Salesforce org".to_string(),
        ],
        completion_benefits: vec![
            "Real-time architecture analysis and recommendations".to_string(),
            "Automated governance and compliance monitoring".to_string(),
            "Performance optimization suggestions".to_string(),
            "Integration pattern recommendations".to_string(),
            "Security assessment and hardening advice".to_string(),
        ],
    }
}

fn create_salesforce_dashboard() -> DashboardConfig {
    DashboardConfig {
        layout: DashboardLayout::Grid,
        widgets: vec![
            DashboardWidget {
                widget_id: "sf_org_health".to_string(),
                title: "Org Health Score".to_string(),
                widget_type: WidgetType::StatusCard,
                position: WidgetPosition { row: 1, column: 1, order: 1 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_trends": true,
                    "include_recommendations": true,
                    "color_coding": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "sf_performance_metrics".to_string(),
                title: "Performance Metrics".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 1, column: 2, order: 2 },
                size: WidgetSize::Large,
                config: json!({
                    "chart_type": "line",
                    "time_range": "30_days",
                    "metrics": ["page_load_time", "api_response_time", "user_sessions"]
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "sf_architecture_insights".to_string(),
                title: "Architecture Insights".to_string(),
                widget_type: WidgetType::AlertsPanel,
                position: WidgetPosition { row: 2, column: 1, order: 3 },
                size: WidgetSize::Wide,
                config: json!({
                    "max_items": 5,
                    "priority_filter": ["high", "critical"],
                    "auto_refresh": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "sf_recent_analyses".to_string(),
                title: "Recent Architecture Analyses".to_string(),
                widget_type: WidgetType::RecentActivity,
                position: WidgetPosition { row: 3, column: 1, order: 4 },
                size: WidgetSize::Medium,
                config: json!({
                    "item_count": 10,
                    "show_timestamps": true,
                    "include_status": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "sf_integration_status".to_string(),
                title: "Integration Status".to_string(),
                widget_type: WidgetType::QuickStats,
                position: WidgetPosition { row: 3, column: 2, order: 5 },
                size: WidgetSize::Medium,
                config: json!({
                    "stats": [
                        {"label": "Active Integrations", "key": "active_count"},
                        {"label": "Failed Connections", "key": "failed_count"},
                        {"label": "Avg Response Time", "key": "avg_response_ms"}
                    ]
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "sf_task_queue".to_string(),
                title: "Analysis Queue".to_string(),
                widget_type: WidgetType::TaskQueue,
                position: WidgetPosition { row: 4, column: 1, order: 6 },
                size: WidgetSize::Wide,
                config: json!({
                    "show_progress": true,
                    "allow_prioritization": true,
                    "max_visible": 8
                }),
                visible: true,
            },
        ],
        refresh_interval_seconds: 30,
        auto_refresh: true,
        show_welcome: true,
    }
}

fn create_salesforce_quick_actions() -> Vec<QuickAction> {
    vec![
        QuickAction {
            action_id: "sf_architecture_review".to_string(),
            label: "Run Architecture Review".to_string(),
            description: "Perform comprehensive architecture analysis of your Salesforce org".to_string(),
            icon: "🔍".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/salesforce/architecture-review".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+R".to_string()),
        },
        QuickAction {
            action_id: "sf_performance_analysis".to_string(),
            label: "Analyze Performance".to_string(),
            description: "Deep-dive performance analysis with optimization recommendations".to_string(),
            icon: "⚡".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/salesforce/performance-analysis".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+P".to_string()),
        },
        QuickAction {
            action_id: "sf_security_assessment".to_string(),
            label: "Security Assessment".to_string(),
            description: "Comprehensive security review and vulnerability assessment".to_string(),
            icon: "🛡️".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/salesforce/security-assessment".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+S".to_string()),
        },
        QuickAction {
            action_id: "sf_integration_design".to_string(),
            label: "Design Integration".to_string(),
            description: "Launch integration design wizard for new system connections".to_string(),
            icon: "🔗".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/wizards/integration-design".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+I".to_string()),
        },
        QuickAction {
            action_id: "sf_generate_report".to_string(),
            label: "Generate Report".to_string(),
            description: "Create comprehensive architecture and recommendations report".to_string(),
            icon: "📊".to_string(),
            action_type: ActionType::Download,
            endpoint: "/api/salesforce/generate-report".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+G".to_string()),
        },
        QuickAction {
            action_id: "sf_configure_alerts".to_string(),
            label: "Configure Alerts".to_string(),
            description: "Set up monitoring alerts and notification preferences".to_string(),
            icon: "🔔".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/salesforce-cta/alerts".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+A".to_string()),
        },
    ]
}

fn create_salesforce_navigation() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            item_id: "sf_dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            url: "/portals/salesforce-cta".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "sf_architecture".to_string(),
            label: "Architecture".to_string(),
            icon: "🏗️".to_string(),
            url: "/portals/salesforce-cta/architecture".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "sf_arch_review".to_string(),
                    label: "Review & Assessment".to_string(),
                    icon: "🔍".to_string(),
                    url: "/portals/salesforce-cta/architecture/review".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "sf_arch_patterns".to_string(),
                    label: "Solution Patterns".to_string(),
                    icon: "🎯".to_string(),
                    url: "/portals/salesforce-cta/architecture/patterns".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "sf_arch_governance".to_string(),
                    label: "Governance".to_string(),
                    icon: "⚖️".to_string(),
                    url: "/portals/salesforce-cta/architecture/governance".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "sf_integrations".to_string(),
            label: "Integrations".to_string(),
            icon: "🔗".to_string(),
            url: "/portals/salesforce-cta/integrations".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: Some(3), // Active integration issues
            children: vec![
                NavigationItem {
                    item_id: "sf_int_active".to_string(),
                    label: "Active Connections".to_string(),
                    icon: "✅".to_string(),
                    url: "/portals/salesforce-cta/integrations/active".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "sf_int_design".to_string(),
                    label: "Design New".to_string(),
                    icon: "➕".to_string(),
                    url: "/portals/salesforce-cta/integrations/design".to_string(),
                    target: NavigationTarget::Modal,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "sf_int_monitoring".to_string(),
                    label: "Monitoring".to_string(),
                    icon: "📈".to_string(),
                    url: "/portals/salesforce-cta/integrations/monitoring".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "sf_security".to_string(),
            label: "Security".to_string(),
            icon: "🛡️".to_string(),
            url: "/portals/salesforce-cta/security".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("security_admin".to_string()),
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "sf_sec_assessment".to_string(),
                    label: "Security Assessment".to_string(),
                    icon: "🔒".to_string(),
                    url: "/portals/salesforce-cta/security/assessment".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("security_admin".to_string()),
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "sf_sec_compliance".to_string(),
                    label: "Compliance Check".to_string(),
                    icon: "✔️".to_string(),
                    url: "/portals/salesforce-cta/security/compliance".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("security_admin".to_string()),
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "sf_performance".to_string(),
            label: "Performance".to_string(),
            icon: "⚡".to_string(),
            url: "/portals/salesforce-cta/performance".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "sf_reports".to_string(),
            label: "Reports".to_string(),
            icon: "📋".to_string(),
            url: "/portals/salesforce-cta/reports".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "sf_settings".to_string(),
            label: "Settings".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/salesforce-cta/settings".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("admin".to_string()),
            badge_count: None,
            children: vec![],
        },
    ]
}

fn create_salesforce_help_resources() -> Vec<HelpResource> {
    vec![
        HelpResource {
            resource_id: "sf_getting_started".to_string(),
            title: "Getting Started with Salesforce CTA Bot".to_string(),
            description: "Complete guide to setting up and using the Salesforce CTA Portal".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/salesforce-cta/getting-started".to_string(),
            category: "Setup".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(15),
        },
        HelpResource {
            resource_id: "sf_architecture_patterns".to_string(),
            title: "Salesforce Architecture Patterns Guide".to_string(),
            description: "Comprehensive guide to enterprise Salesforce architecture patterns and best practices".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/salesforce-cta/architecture-patterns".to_string(),
            category: "Architecture".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(45),
        },
        HelpResource {
            resource_id: "sf_integration_howto".to_string(),
            title: "Integration Design Best Practices".to_string(),
            description: "Step-by-step guide for designing robust Salesforce integrations".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/salesforce-cta/integration-guide".to_string(),
            category: "Integrations".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(30),
        },
        HelpResource {
            resource_id: "sf_security_checklist".to_string(),
            title: "Salesforce Security Checklist".to_string(),
            description: "Essential security configurations and compliance requirements".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/salesforce-cta/security-checklist".to_string(),
            category: "Security".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(20),
        },
        HelpResource {
            resource_id: "sf_performance_tuning".to_string(),
            title: "Performance Optimization Techniques".to_string(),
            description: "Advanced techniques for optimizing Salesforce org performance".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/salesforce-cta/performance-tuning".to_string(),
            category: "Performance".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: Some(60),
        },
        HelpResource {
            resource_id: "sf_troubleshooting".to_string(),
            title: "Common Issues & Troubleshooting".to_string(),
            description: "Solutions to frequently encountered problems and error messages".to_string(),
            resource_type: HelpResourceType::Troubleshooting,
            url: "/help/salesforce-cta/troubleshooting".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(25),
        },
        HelpResource {
            resource_id: "sf_api_reference".to_string(),
            title: "Salesforce CTA API Reference".to_string(),
            description: "Complete API documentation for developers and integrators".to_string(),
            resource_type: HelpResourceType::ApiReference,
            url: "/api/docs/salesforce-cta".to_string(),
            category: "Developer".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: None,
        },
        HelpResource {
            resource_id: "sf_video_walkthrough".to_string(),
            title: "Portal Walkthrough Video".to_string(),
            description: "15-minute video tour of all portal features and capabilities".to_string(),
            resource_type: HelpResourceType::Video,
            url: "/videos/salesforce-cta-walkthrough".to_string(),
            category: "Overview".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(15),
        },
        HelpResource {
            resource_id: "sf_faq".to_string(),
            title: "Frequently Asked Questions".to_string(),
            description: "Answers to the most common questions about the Salesforce CTA Portal".to_string(),
            resource_type: HelpResourceType::FAQ,
            url: "/help/salesforce-cta/faq".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(10),
        },
    ]
}
