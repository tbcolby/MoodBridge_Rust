use super::*;
use crate::bots::BotSpecialty;
use crate::wizard::WizardType;
use chrono::Utc;
use serde_json::json;

/// Create the Deadline Management Portal configuration
pub fn create_deadline_management_portal() -> BotPortal {
    BotPortal {
        portal_id: "deadline_management".to_string(),
        bot_specialty: BotSpecialty::DeadlineManagement,
        name: "Deadline Management Portal".to_string(),
        description: "Comprehensive deadline tracking with intelligent alerts, crisis management, and automated calendar integration. Never miss a critical deadline again.".to_string(),
        icon: "⏰".to_string(),
        color_scheme: ColorScheme {
            primary: "#dc2626".to_string(),    // Red
            secondary: "#991b1b".to_string(),   // Dark Red
            accent: "#f59e0b".to_string(),      // Amber
            background: "#fef2f2".to_string(),  // Light Red
            text: "#1f2937".to_string(),        // Gray
            success: "#059669".to_string(),     // Emerald
            warning: "#d97706".to_string(),     // Orange
            error: "#dc2626".to_string(),       // Red
        },
        status: PortalStatus::Active,
        setup_wizard: create_deadline_setup_wizard(),
        dashboard_config: create_deadline_dashboard(),
        quick_actions: create_deadline_quick_actions(),
        navigation_menu: create_deadline_navigation(),
        help_resources: create_deadline_help_resources(),
        created_at: Utc::now(),
        last_accessed: None,
    }
}

fn create_deadline_setup_wizard() -> SetupWizardConfig {
    SetupWizardConfig {
        wizard_id: "deadline_management_setup".to_string(),
        wizard_type: WizardType::DeadlineManagement,
        required_steps: vec![
            WizardStepConfig {
                step_id: "deadline_calendar_integration".to_string(),
                title: "Calendar Integration".to_string(),
                description: "Connect to calendar systems and scheduling platforms".to_string(),
                step_type: WizardStepType::Integration,
                required: true,
                estimated_minutes: 10,
                help_text: Some("Integrate with Outlook, Google Calendar, and case management systems".to_string()),
            },
            WizardStepConfig {
                step_id: "deadline_categories_setup".to_string(),
                title: "Deadline Categories & Rules".to_string(),
                description: "Define deadline types, priority levels, and automatic rules".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 15,
                help_text: Some("Set up court deadlines, filing deadlines, and custom deadline types".to_string()),
            },
            WizardStepConfig {
                step_id: "deadline_alert_config".to_string(),
                title: "Alert Configuration".to_string(),
                description: "Configure notification schedules and escalation procedures".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 12,
                help_text: Some("Set up multi-level alerts and crisis management protocols".to_string()),
            },
            WizardStepConfig {
                step_id: "deadline_team_setup".to_string(),
                title: "Team & Responsibility Assignment".to_string(),
                description: "Configure team members, roles, and responsibility matrices".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 8,
                help_text: Some("Define who gets notified and who is responsible for each deadline type".to_string()),
            },
            WizardStepConfig {
                step_id: "deadline_automation_rules".to_string(),
                title: "Automation Rules".to_string(),
                description: "Set up automated deadline calculation and tracking rules".to_string(),
                step_type: WizardStepType::Configuration,
                required: false,
                estimated_minutes: 10,
                help_text: Some("Configure rules for automatic deadline creation from case events".to_string()),
            },
        ],
        estimated_time_minutes: 55,
        prerequisites: vec![
            "Access to calendar systems and case management platforms".to_string(),
            "List of deadline types and court rules".to_string(),
            "Team member contact information and roles".to_string(),
        ],
        completion_benefits: vec![
            "Automated deadline tracking and alerts".to_string(),
            "Multi-level notification system with escalation".to_string(),
            "Integration with existing calendar and case management systems".to_string(),
            "Crisis management for missed or at-risk deadlines".to_string(),
            "Comprehensive deadline analytics and reporting".to_string(),
        ],
    }
}

fn create_deadline_dashboard() -> DashboardConfig {
    DashboardConfig {
        layout: DashboardLayout::Grid,
        widgets: vec![
            DashboardWidget {
                widget_id: "deadline_urgency_overview".to_string(),
                title: "Urgency Overview".to_string(),
                widget_type: WidgetType::StatusCard,
                position: WidgetPosition { row: 1, column: 1, order: 1 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_critical": true,
                    "show_upcoming": true,
                    "color_coding": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_calendar_view".to_string(),
                title: "Deadline Calendar".to_string(),
                widget_type: WidgetType::Calendar,
                position: WidgetPosition { row: 1, column: 2, order: 2 },
                size: WidgetSize::Large,
                config: json!({
                    "view_type": "month",
                    "show_details": true,
                    "interactive": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_critical_alerts".to_string(),
                title: "Critical Alerts".to_string(),
                widget_type: WidgetType::AlertsPanel,
                position: WidgetPosition { row: 2, column: 1, order: 3 },
                size: WidgetSize::Medium,
                config: json!({
                    "priority_filter": ["critical", "high"],
                    "auto_refresh": true,
                    "max_items": 5
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_upcoming_tasks".to_string(),
                title: "Upcoming Deadlines".to_string(),
                widget_type: WidgetType::TaskQueue,
                position: WidgetPosition { row: 2, column: 2, order: 4 },
                size: WidgetSize::Medium,
                config: json!({
                    "sort_by": "date",
                    "show_countdown": true,
                    "max_items": 8
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_completion_stats".to_string(),
                title: "Completion Statistics".to_string(),
                widget_type: WidgetType::DonutChart,
                position: WidgetPosition { row: 3, column: 1, order: 5 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "donut",
                    "show_percentages": true,
                    "categories": ["completed", "pending", "overdue"]
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_team_workload".to_string(),
                title: "Team Workload".to_string(),
                widget_type: WidgetType::BarChart,
                position: WidgetPosition { row: 3, column: 2, order: 6 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "horizontal_bar",
                    "show_values": true,
                    "group_by": "team_member"
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "deadline_trend_analysis".to_string(),
                title: "Deadline Trends".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 4, column: 1, order: 7 },
                size: WidgetSize::Wide,
                config: json!({
                    "chart_type": "line",
                    "time_range": "30_days",
                    "metrics": ["created", "completed", "missed"]
                }),
                visible: true,
            },
        ],
        refresh_interval_seconds: 60, // More frequent refresh for deadlines
        auto_refresh: true,
        show_welcome: true,
    }
}

fn create_deadline_quick_actions() -> Vec<QuickAction> {
    vec![
        QuickAction {
            action_id: "deadline_create_urgent".to_string(),
            label: "Add Urgent Deadline".to_string(),
            description: "Quickly add a critical deadline with immediate alerts".to_string(),
            icon: "🚨".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/deadlines/create-urgent".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+U".to_string()),
        },
        QuickAction {
            action_id: "deadline_bulk_import".to_string(),
            label: "Bulk Import".to_string(),
            description: "Import deadlines from calendar or case management systems".to_string(),
            icon: "📥".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/deadlines/bulk-import".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+I".to_string()),
        },
        QuickAction {
            action_id: "deadline_crisis_mode".to_string(),
            label: "Crisis Mode".to_string(),
            description: "Activate crisis management for missed or at-risk deadlines".to_string(),
            icon: "🆘".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/deadlines/crisis-mode".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+C".to_string()),
        },
        QuickAction {
            action_id: "deadline_calendar_sync".to_string(),
            label: "Sync Calendars".to_string(),
            description: "Force synchronization with all connected calendar systems".to_string(),
            icon: "🔄".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/deadlines/sync-calendars".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+S".to_string()),
        },
        QuickAction {
            action_id: "deadline_generate_report".to_string(),
            label: "Generate Report".to_string(),
            description: "Create comprehensive deadline management report".to_string(),
            icon: "📊".to_string(),
            action_type: ActionType::Download,
            endpoint: "/api/deadlines/reports/generate".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+R".to_string()),
        },
        QuickAction {
            action_id: "deadline_team_dashboard".to_string(),
            label: "Team Dashboard".to_string(),
            description: "View team-wide deadline assignments and workload".to_string(),
            icon: "👥".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/deadline-management/team".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+T".to_string()),
        },
    ]
}

fn create_deadline_navigation() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            item_id: "deadline_dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            url: "/portals/deadline-management".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "deadline_calendar".to_string(),
            label: "Calendar View".to_string(),
            icon: "📅".to_string(),
            url: "/portals/deadline-management/calendar".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "deadline_calendar_month".to_string(),
                    label: "Month View".to_string(),
                    icon: "📅".to_string(),
                    url: "/portals/deadline-management/calendar/month".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "deadline_calendar_week".to_string(),
                    label: "Week View".to_string(),
                    icon: "📆".to_string(),
                    url: "/portals/deadline-management/calendar/week".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "deadline_alerts".to_string(),
            label: "Alerts & Notifications".to_string(),
            icon: "🔔".to_string(),
            url: "/portals/deadline-management/alerts".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: Some(7), // Active alerts
            children: vec![
                NavigationItem {
                    item_id: "deadline_alerts_critical".to_string(),
                    label: "Critical Alerts".to_string(),
                    icon: "🚨".to_string(),
                    url: "/portals/deadline-management/alerts/critical".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: Some(3),
                    children: vec![],
                },
                NavigationItem {
                    item_id: "deadline_alerts_upcoming".to_string(),
                    label: "Upcoming".to_string(),
                    icon: "⏳".to_string(),
                    url: "/portals/deadline-management/alerts/upcoming".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "deadline_team".to_string(),
            label: "Team Management".to_string(),
            icon: "👥".to_string(),
            url: "/portals/deadline-management/team".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "deadline_analytics".to_string(),
            label: "Analytics".to_string(),
            icon: "📈".to_string(),
            url: "/portals/deadline-management/analytics".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "deadline_settings".to_string(),
            label: "Settings".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/deadline-management/settings".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("admin".to_string()),
            badge_count: None,
            children: vec![],
        },
    ]
}

fn create_deadline_help_resources() -> Vec<HelpResource> {
    vec![
        HelpResource {
            resource_id: "deadline_getting_started".to_string(),
            title: "Getting Started with Deadline Management".to_string(),
            description: "Complete guide to setting up and using the deadline management system".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/deadline-management/getting-started".to_string(),
            category: "Setup".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(12),
        },
        HelpResource {
            resource_id: "deadline_crisis_management".to_string(),
            title: "Crisis Management Best Practices".to_string(),
            description: "How to handle missed deadlines and crisis situations".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/deadline-management/crisis-management".to_string(),
            category: "Crisis Management".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(18),
        },
        HelpResource {
            resource_id: "deadline_court_rules".to_string(),
            title: "Court Rules & Deadline Calculations".to_string(),
            description: "Understanding court rules and automatic deadline calculations".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/deadline-management/court-rules".to_string(),
            category: "Legal Rules".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(30),
        },
        HelpResource {
            resource_id: "deadline_integration_guide".to_string(),
            title: "Calendar Integration Guide".to_string(),
            description: "Step-by-step guide for integrating with calendar systems".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/deadline-management/integration".to_string(),
            category: "Integration".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(20),
        },
        HelpResource {
            resource_id: "deadline_api_reference".to_string(),
            title: "Deadline Management API Reference".to_string(),
            description: "Complete API documentation for developers and integrators".to_string(),
            resource_type: HelpResourceType::ApiReference,
            url: "/api/docs/deadline-management".to_string(),
            category: "Developer".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: None,
        },
        HelpResource {
            resource_id: "deadline_troubleshooting".to_string(),
            title: "Troubleshooting Deadline Issues".to_string(),
            description: "Solutions to common deadline management problems".to_string(),
            resource_type: HelpResourceType::Troubleshooting,
            url: "/help/deadline-management/troubleshooting".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(15),
        },
    ]
}
