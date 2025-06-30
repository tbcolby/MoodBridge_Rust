use super::*;
use crate::bots::BotSpecialty;
use crate::wizard::WizardType;
use chrono::Utc;
use serde_json::json;

/// Create the Email Notification Portal configuration
pub fn create_email_notification_portal() -> BotPortal {
    BotPortal {
        portal_id: "email_notification".to_string(),
        bot_specialty: BotSpecialty::EmailNotification,
        name: "Email Notification Portal".to_string(),
        description: "Intelligent email automation with smart templates, delivery optimization, and comprehensive tracking. Streamline all your email communications.".to_string(),
        icon: "📧".to_string(),
        color_scheme: ColorScheme {
            primary: "#2563eb".to_string(),    // Blue
            secondary: "#1d4ed8".to_string(),   // Dark Blue
            accent: "#10b981".to_string(),      // Emerald
            background: "#eff6ff".to_string(),  // Light Blue
            text: "#1f2937".to_string(),        // Gray
            success: "#059669".to_string(),     // Emerald
            warning: "#d97706".to_string(),     // Orange
            error: "#dc2626".to_string(),       // Red
        },
        status: PortalStatus::Active,
        setup_wizard: create_email_setup_wizard(),
        dashboard_config: create_email_dashboard(),
        quick_actions: create_email_quick_actions(),
        navigation_menu: create_email_navigation(),
        help_resources: create_email_help_resources(),
        created_at: Utc::now(),
        last_accessed: None,
    }
}

fn create_email_setup_wizard() -> SetupWizardConfig {
    SetupWizardConfig {
        wizard_id: "email_notification_setup".to_string(),
        wizard_type: WizardType::EmailNotification,
        required_steps: vec![
            WizardStepConfig {
                step_id: "email_server_config".to_string(),
                title: "Email Server Configuration".to_string(),
                description: "Configure SMTP servers and email service providers".to_string(),
                step_type: WizardStepType::Integration,
                required: true,
                estimated_minutes: 15,
                help_text: Some("Set up connections to Office 365, Gmail, or custom SMTP servers".to_string()),
            },
            WizardStepConfig {
                step_id: "email_templates_setup".to_string(),
                title: "Email Templates & Branding".to_string(),
                description: "Create email templates and configure organizational branding".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 20,
                help_text: Some("Design professional templates with firm branding and legal disclaimers".to_string()),
            },
            WizardStepConfig {
                step_id: "email_automation_rules".to_string(),
                title: "Automation Rules".to_string(),
                description: "Set up triggers and automation rules for email notifications".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 18,
                help_text: Some("Configure automatic notifications for case updates, deadlines, and events".to_string()),
            },
            WizardStepConfig {
                step_id: "email_delivery_settings".to_string(),
                title: "Delivery & Security Settings".to_string(),
                description: "Configure delivery options, security, and compliance settings".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 12,
                help_text: Some("Set up encryption, delivery scheduling, and compliance monitoring".to_string()),
            },
            WizardStepConfig {
                step_id: "email_tracking_analytics".to_string(),
                title: "Tracking & Analytics".to_string(),
                description: "Enable email tracking and analytics features".to_string(),
                step_type: WizardStepType::Configuration,
                required: false,
                estimated_minutes: 8,
                help_text: Some("Configure read receipts, click tracking, and engagement analytics".to_string()),
            },
        ],
        estimated_time_minutes: 73,
        prerequisites: vec![
            "Access to email server credentials and settings".to_string(),
            "Firm branding assets (logo, colors, fonts)".to_string(),
            "Legal disclaimer text and compliance requirements".to_string(),
            "List of automation triggers and notification requirements".to_string(),
        ],
        completion_benefits: vec![
            "Automated email notifications for all system events".to_string(),
            "Professional branded email templates".to_string(),
            "Advanced delivery optimization and scheduling".to_string(),
            "Comprehensive email tracking and analytics".to_string(),
            "Compliance monitoring and audit trails".to_string(),
        ],
    }
}

fn create_email_dashboard() -> DashboardConfig {
    DashboardConfig {
        layout: DashboardLayout::Grid,
        widgets: vec![
            DashboardWidget {
                widget_id: "email_delivery_overview".to_string(),
                title: "Delivery Overview".to_string(),
                widget_type: WidgetType::StatusCard,
                position: WidgetPosition { row: 1, column: 1, order: 1 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_sent": true,
                    "show_pending": true,
                    "show_failed": true,
                    "real_time": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_queue_status".to_string(),
                title: "Email Queue".to_string(),
                widget_type: WidgetType::TaskQueue,
                position: WidgetPosition { row: 1, column: 2, order: 2 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_priority": true,
                    "show_scheduled": true,
                    "max_items": 10
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_templates_usage".to_string(),
                title: "Template Usage".to_string(),
                widget_type: WidgetType::DonutChart,
                position: WidgetPosition { row: 1, column: 3, order: 3 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "donut",
                    "show_percentages": true,
                    "top_templates": 5
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_engagement_metrics".to_string(),
                title: "Engagement Metrics".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 2, column: 1, order: 4 },
                size: WidgetSize::Wide,
                config: json!({
                    "chart_type": "line",
                    "metrics": ["open_rate", "click_rate", "response_rate"],
                    "time_range": "7_days"
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_delivery_volume".to_string(),
                title: "Daily Volume".to_string(),
                widget_type: WidgetType::BarChart,
                position: WidgetPosition { row: 2, column: 3, order: 5 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "bar",
                    "time_range": "30_days",
                    "show_trend": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_failed_deliveries".to_string(),
                title: "Failed Deliveries".to_string(),
                widget_type: WidgetType::AlertsPanel,
                position: WidgetPosition { row: 3, column: 1, order: 6 },
                size: WidgetSize::Medium,
                config: json!({
                    "severity_filter": ["error", "warning"],
                    "auto_refresh": true,
                    "max_items": 8
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_automation_status".to_string(),
                title: "Automation Rules".to_string(),
                widget_type: WidgetType::StatusCard,
                position: WidgetPosition { row: 3, column: 2, order: 7 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_active": true,
                    "show_paused": true,
                    "show_statistics": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "email_security_alerts".to_string(),
                title: "Security & Compliance".to_string(),
                widget_type: WidgetType::AlertsPanel,
                position: WidgetPosition { row: 3, column: 3, order: 8 },
                size: WidgetSize::Medium,
                config: json!({
                    "alert_types": ["security", "compliance"],
                    "severity_threshold": "medium",
                    "max_items": 5
                }),
                visible: true,
            },
        ],
        refresh_interval_seconds: 120,
        auto_refresh: true,
        show_welcome: true,
    }
}

fn create_email_quick_actions() -> Vec<QuickAction> {
    vec![
        QuickAction {
            action_id: "email_compose_quick".to_string(),
            label: "Compose Email".to_string(),
            description: "Quickly compose and send an email using templates".to_string(),
            icon: "✉️".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/emails/compose".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+N".to_string()),
        },
        QuickAction {
            action_id: "email_broadcast".to_string(),
            label: "Send Broadcast".to_string(),
            description: "Send emails to multiple recipients using templates".to_string(),
            icon: "📢".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/emails/broadcast".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+B".to_string()),
        },
        QuickAction {
            action_id: "email_template_create".to_string(),
            label: "Create Template".to_string(),
            description: "Design a new email template with branding".to_string(),
            icon: "🎨".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/emails/templates/create".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+T".to_string()),
        },
        QuickAction {
            action_id: "email_queue_manage".to_string(),
            label: "Manage Queue".to_string(),
            description: "View and manage the email delivery queue".to_string(),
            icon: "📋".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/email-notification/queue".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+Q".to_string()),
        },
        QuickAction {
            action_id: "email_analytics_report".to_string(),
            label: "Analytics Report".to_string(),
            description: "Generate comprehensive email analytics report".to_string(),
            icon: "📊".to_string(),
            action_type: ActionType::Download,
            endpoint: "/api/emails/reports/analytics".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+A".to_string()),
        },
        QuickAction {
            action_id: "email_test_delivery".to_string(),
            label: "Test Delivery".to_string(),
            description: "Send test emails to verify configuration".to_string(),
            icon: "🧪".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/emails/test".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+E".to_string()),
        },
    ]
}

fn create_email_navigation() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            item_id: "email_dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            url: "/portals/email-notification".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "email_compose".to_string(),
            label: "Compose".to_string(),
            icon: "✏️".to_string(),
            url: "/portals/email-notification/compose".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "email_compose_new".to_string(),
                    label: "New Email".to_string(),
                    icon: "✉️".to_string(),
                    url: "/portals/email-notification/compose/new".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "email_compose_broadcast".to_string(),
                    label: "Broadcast".to_string(),
                    icon: "📢".to_string(),
                    url: "/portals/email-notification/compose/broadcast".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("email_broadcast".to_string()),
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "email_templates".to_string(),
            label: "Templates".to_string(),
            icon: "📝".to_string(),
            url: "/portals/email-notification/templates".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "email_templates_library".to_string(),
                    label: "Template Library".to_string(),
                    icon: "📚".to_string(),
                    url: "/portals/email-notification/templates/library".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "email_templates_builder".to_string(),
                    label: "Template Builder".to_string(),
                    icon: "🎨".to_string(),
                    url: "/portals/email-notification/templates/builder".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("template_edit".to_string()),
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "email_queue".to_string(),
            label: "Queue Management".to_string(),
            icon: "📋".to_string(),
            url: "/portals/email-notification/queue".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: Some(24), // Pending emails
            children: vec![],
        },
        NavigationItem {
            item_id: "email_automation".to_string(),
            label: "Automation".to_string(),
            icon: "🤖".to_string(),
            url: "/portals/email-notification/automation".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("automation_manage".to_string()),
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "email_analytics".to_string(),
            label: "Analytics".to_string(),
            icon: "📈".to_string(),
            url: "/portals/email-notification/analytics".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "email_settings".to_string(),
            label: "Settings".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/email-notification/settings".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("admin".to_string()),
            badge_count: None,
            children: vec![],
        },
    ]
}

fn create_email_help_resources() -> Vec<HelpResource> {
    vec![
        HelpResource {
            resource_id: "email_getting_started".to_string(),
            title: "Getting Started with Email Notifications".to_string(),
            description: "Complete setup guide for email notification system".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/email-notification/getting-started".to_string(),
            category: "Setup".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(15),
        },
        HelpResource {
            resource_id: "email_template_design".to_string(),
            title: "Email Template Design Guide".to_string(),
            description: "Best practices for creating professional email templates".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/email-notification/template-design".to_string(),
            category: "Templates".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(25),
        },
        HelpResource {
            resource_id: "email_automation_setup".to_string(),
            title: "Setting Up Email Automation".to_string(),
            description: "Configure triggers and automation rules for email notifications".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/email-notification/automation".to_string(),
            category: "Automation".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(20),
        },
        HelpResource {
            resource_id: "email_deliverability".to_string(),
            title: "Email Deliverability Best Practices".to_string(),
            description: "Optimize email delivery rates and avoid spam filters".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/email-notification/deliverability".to_string(),
            category: "Delivery".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(22),
        },
        HelpResource {
            resource_id: "email_compliance".to_string(),
            title: "Email Compliance & Security".to_string(),
            description: "Legal compliance requirements and security best practices".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/email-notification/compliance".to_string(),
            category: "Compliance".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(30),
        },
        HelpResource {
            resource_id: "email_api_reference".to_string(),
            title: "Email API Reference".to_string(),
            description: "Complete API documentation for email notification system".to_string(),
            resource_type: HelpResourceType::ApiReference,
            url: "/api/docs/email-notification".to_string(),
            category: "Developer".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: None,
        },
        HelpResource {
            resource_id: "email_troubleshooting".to_string(),
            title: "Email Troubleshooting Guide".to_string(),
            description: "Solutions to common email delivery and configuration issues".to_string(),
            resource_type: HelpResourceType::Troubleshooting,
            url: "/help/email-notification/troubleshooting".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(18),
        },
    ]
}
