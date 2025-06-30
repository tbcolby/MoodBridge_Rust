use super::*;
use crate::bots::BotSpecialty;
use crate::wizard::WizardType;
use chrono::Utc;
use serde_json::json;

/// Create the Analytics Reporting Portal configuration
pub fn create_analytics_reporting_portal() -> BotPortal {
    BotPortal {
        portal_id: "analytics_reporting".to_string(),
        bot_specialty: BotSpecialty::AnalyticsReporting,
        name: "Analytics Reporting Portal".to_string(),
        description: "Advanced analytics with interactive dashboards, custom reports, and AI-powered insights. Transform your data into actionable intelligence.".to_string(),
        icon: "📊".to_string(),
        color_scheme: ColorScheme {
            primary: "#7c3aed".to_string(),    // Purple
            secondary: "#5b21b6".to_string(),   // Dark Purple
            accent: "#06b6d4".to_string(),      // Cyan
            background: "#faf5ff".to_string(),  // Light Purple
            text: "#1f2937".to_string(),        // Gray
            success: "#059669".to_string(),     // Emerald
            warning: "#d97706".to_string(),     // Orange
            error: "#dc2626".to_string(),       // Red
        },
        status: PortalStatus::Active,
        setup_wizard: create_analytics_setup_wizard(),
        dashboard_config: create_analytics_dashboard(),
        quick_actions: create_analytics_quick_actions(),
        navigation_menu: create_analytics_navigation(),
        help_resources: create_analytics_help_resources(),
        created_at: Utc::now(),
        last_accessed: None,
    }
}

fn create_analytics_setup_wizard() -> SetupWizardConfig {
    SetupWizardConfig {
        wizard_id: "analytics_reporting_setup".to_string(),
        wizard_type: WizardType::AnalyticsReporting,
        required_steps: vec![
            WizardStepConfig {
                step_id: "analytics_data_sources".to_string(),
                title: "Data Source Configuration".to_string(),
                description: "Connect to databases, APIs, and external data sources".to_string(),
                step_type: WizardStepType::Integration,
                required: true,
                estimated_minutes: 25,
                help_text: Some("Configure connections to case management, billing, time tracking, and external systems".to_string()),
            },
            WizardStepConfig {
                step_id: "analytics_kpi_setup".to_string(),
                title: "KPI & Metrics Definition".to_string(),
                description: "Define key performance indicators and business metrics".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 20,
                help_text: Some("Set up metrics for case performance, team productivity, and financial indicators".to_string()),
            },
            WizardStepConfig {
                step_id: "analytics_dashboard_design".to_string(),
                title: "Dashboard Design".to_string(),
                description: "Create custom dashboards and visualization layouts".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 30,
                help_text: Some("Design executive, operational, and team-specific dashboard views".to_string()),
            },
            WizardStepConfig {
                step_id: "analytics_report_templates".to_string(),
                title: "Report Templates".to_string(),
                description: "Configure automated report templates and schedules".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 18,
                help_text: Some("Set up weekly, monthly, and quarterly report templates".to_string()),
            },
            WizardStepConfig {
                step_id: "analytics_alerts_thresholds".to_string(),
                title: "Alerts & Thresholds".to_string(),
                description: "Configure performance alerts and threshold monitoring".to_string(),
                step_type: WizardStepType::Configuration,
                required: false,
                estimated_minutes: 12,
                help_text: Some("Set up alerts for KPI deviations and performance issues".to_string()),
            },
            WizardStepConfig {
                step_id: "analytics_user_permissions".to_string(),
                title: "User Access & Permissions".to_string(),
                description: "Configure user access levels and data permissions".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 15,
                help_text: Some("Define who can access what data and reports".to_string()),
            },
        ],
        estimated_time_minutes: 120,
        prerequisites: vec![
            "Access to all data sources and database credentials".to_string(),
            "List of required KPIs and business metrics".to_string(),
            "Stakeholder requirements for dashboards and reports".to_string(),
            "User roles and access level definitions".to_string(),
            "Data governance and security policies".to_string(),
        ],
        completion_benefits: vec![
            "Real-time dashboards with interactive visualizations".to_string(),
            "Automated report generation and distribution".to_string(),
            "AI-powered insights and trend analysis".to_string(),
            "Performance monitoring with intelligent alerts".to_string(),
            "Data-driven decision making capabilities".to_string(),
        ],
    }
}

fn create_analytics_dashboard() -> DashboardConfig {
    DashboardConfig {
        layout: DashboardLayout::Grid,
        widgets: vec![
            DashboardWidget {
                widget_id: "analytics_kpi_overview".to_string(),
                title: "Key Performance Indicators".to_string(),
                widget_type: WidgetType::KpiCard,
                position: WidgetPosition { row: 1, column: 1, order: 1 },
                size: WidgetSize::Wide,
                config: json!({
                    "kpis": ["case_volume", "billable_hours", "client_satisfaction", "revenue"],
                    "show_trends": true,
                    "time_period": "current_month"
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_revenue_trends".to_string(),
                title: "Revenue Trends".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 2, column: 1, order: 2 },
                size: WidgetSize::Large,
                config: json!({
                    "chart_type": "area",
                    "metrics": ["monthly_revenue", "projected_revenue"],
                    "time_range": "12_months",
                    "show_forecast": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_case_performance".to_string(),
                title: "Case Performance".to_string(),
                widget_type: WidgetType::BarChart,
                position: WidgetPosition { row: 2, column: 2, order: 3 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "stacked_bar",
                    "group_by": "practice_area",
                    "metrics": ["open_cases", "closed_cases", "won_cases"]
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_team_productivity".to_string(),
                title: "Team Productivity".to_string(),
                widget_type: WidgetType::Heatmap,
                position: WidgetPosition { row: 3, column: 1, order: 4 },
                size: WidgetSize::Medium,
                config: json!({
                    "heatmap_type": "team_performance",
                    "metrics": ["billable_hours", "utilization_rate"],
                    "time_period": "monthly"
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_client_insights".to_string(),
                title: "Client Insights".to_string(),
                widget_type: WidgetType::DonutChart,
                position: WidgetPosition { row: 3, column: 2, order: 5 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "donut",
                    "breakdown": "client_tier",
                    "metric": "revenue_contribution",
                    "show_percentages": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_data_quality".to_string(),
                title: "Data Quality Score".to_string(),
                widget_type: WidgetType::GaugeChart,
                position: WidgetPosition { row: 3, column: 3, order: 6 },
                size: WidgetSize::Medium,
                config: json!({
                    "gauge_type": "data_quality",
                    "target_score": 95,
                    "color_ranges": {"good": 90, "warning": 75, "critical": 60}
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_report_schedule".to_string(),
                title: "Scheduled Reports".to_string(),
                widget_type: WidgetType::TaskQueue,
                position: WidgetPosition { row: 4, column: 1, order: 7 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_next_run": true,
                    "show_status": true,
                    "max_items": 6
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "analytics_system_performance".to_string(),
                title: "System Performance".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 4, column: 2, order: 8 },
                size: WidgetSize::Wide,
                config: json!({
                    "chart_type": "line",
                    "metrics": ["query_performance", "data_freshness", "user_activity"],
                    "time_range": "24_hours"
                }),
                visible: true,
            },
        ],
        refresh_interval_seconds: 300, // 5 minutes for analytics data
        auto_refresh: true,
        show_welcome: true,
    }
}

fn create_analytics_quick_actions() -> Vec<QuickAction> {
    vec![
        QuickAction {
            action_id: "analytics_create_report".to_string(),
            label: "Create Report".to_string(),
            description: "Generate a custom report with interactive filters".to_string(),
            icon: "📝".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/analytics/reports/create".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+R".to_string()),
        },
        QuickAction {
            action_id: "analytics_export_dashboard".to_string(),
            label: "Export Dashboard".to_string(),
            description: "Export current dashboard as PDF or PowerPoint".to_string(),
            icon: "📤".to_string(),
            action_type: ActionType::Download,
            endpoint: "/api/analytics/export/dashboard".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+E".to_string()),
        },
        QuickAction {
            action_id: "analytics_schedule_report".to_string(),
            label: "Schedule Report".to_string(),
            description: "Set up automated report generation and delivery".to_string(),
            icon: "⏰".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/analytics/reports/schedule".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+S".to_string()),
        },
        QuickAction {
            action_id: "analytics_data_refresh".to_string(),
            label: "Refresh Data".to_string(),
            description: "Force refresh of all data sources and metrics".to_string(),
            icon: "🔄".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/analytics/data/refresh".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+F5".to_string()),
        },
        QuickAction {
            action_id: "analytics_ai_insights".to_string(),
            label: "AI Insights".to_string(),
            description: "Generate AI-powered insights and recommendations".to_string(),
            icon: "🤖".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/analytics/ai/insights".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+I".to_string()),
        },
        QuickAction {
            action_id: "analytics_query_builder".to_string(),
            label: "Query Builder".to_string(),
            description: "Build custom data queries with visual interface".to_string(),
            icon: "🔍".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/analytics-reporting/query-builder".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+Q".to_string()),
        },
    ]
}

fn create_analytics_navigation() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            item_id: "analytics_dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            url: "/portals/analytics-reporting".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "analytics_reports".to_string(),
            label: "Reports".to_string(),
            icon: "📋".to_string(),
            url: "/portals/analytics-reporting/reports".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "analytics_reports_library".to_string(),
                    label: "Report Library".to_string(),
                    icon: "📚".to_string(),
                    url: "/portals/analytics-reporting/reports/library".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "analytics_reports_builder".to_string(),
                    label: "Report Builder".to_string(),
                    icon: "🔧".to_string(),
                    url: "/portals/analytics-reporting/reports/builder".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("report_create".to_string()),
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "analytics_reports_scheduled".to_string(),
                    label: "Scheduled Reports".to_string(),
                    icon: "⏰".to_string(),
                    url: "/portals/analytics-reporting/reports/scheduled".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: Some(12), // Active schedules
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "analytics_data_explorer".to_string(),
            label: "Data Explorer".to_string(),
            icon: "🔍".to_string(),
            url: "/portals/analytics-reporting/data-explorer".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("data_explore".to_string()),
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "analytics_query_builder".to_string(),
                    label: "Query Builder".to_string(),
                    icon: "🛠️".to_string(),
                    url: "/portals/analytics-reporting/data-explorer/query-builder".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("query_create".to_string()),
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "analytics_data_sources".to_string(),
                    label: "Data Sources".to_string(),
                    icon: "🗄️".to_string(),
                    url: "/portals/analytics-reporting/data-explorer/sources".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: Some("data_source_view".to_string()),
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "analytics_insights".to_string(),
            label: "AI Insights".to_string(),
            icon: "🤖".to_string(),
            url: "/portals/analytics-reporting/insights".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: Some(5), // New insights
            children: vec![],
        },
        NavigationItem {
            item_id: "analytics_visualizations".to_string(),
            label: "Visualizations".to_string(),
            icon: "📈".to_string(),
            url: "/portals/analytics-reporting/visualizations".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "analytics_charts".to_string(),
                    label: "Charts".to_string(),
                    icon: "📊".to_string(),
                    url: "/portals/analytics-reporting/visualizations/charts".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "analytics_maps".to_string(),
                    label: "Geographic Maps".to_string(),
                    icon: "🗺️".to_string(),
                    url: "/portals/analytics-reporting/visualizations/maps".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "analytics_settings".to_string(),
            label: "Settings".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/analytics-reporting/settings".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("admin".to_string()),
            badge_count: None,
            children: vec![],
        },
    ]
}

fn create_analytics_help_resources() -> Vec<HelpResource> {
    vec![
        HelpResource {
            resource_id: "analytics_getting_started".to_string(),
            title: "Getting Started with Analytics".to_string(),
            description: "Complete guide to setting up analytics and reporting".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/analytics-reporting/getting-started".to_string(),
            category: "Setup".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(20),
        },
        HelpResource {
            resource_id: "analytics_dashboard_design".to_string(),
            title: "Dashboard Design Best Practices".to_string(),
            description: "Guidelines for creating effective dashboards and visualizations".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/analytics-reporting/dashboard-design".to_string(),
            category: "Design".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(25),
        },
        HelpResource {
            resource_id: "analytics_kpi_setup".to_string(),
            title: "Setting Up KPIs and Metrics".to_string(),
            description: "How to define and track key performance indicators".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/analytics-reporting/kpi-setup".to_string(),
            category: "Metrics".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(18),
        },
        HelpResource {
            resource_id: "analytics_data_modeling".to_string(),
            title: "Data Modeling for Legal Analytics".to_string(),
            description: "Advanced data modeling techniques for legal practice metrics".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/analytics-reporting/data-modeling".to_string(),
            category: "Data Science".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(35),
        },
        HelpResource {
            resource_id: "analytics_ai_insights".to_string(),
            title: "AI-Powered Insights Guide".to_string(),
            description: "Understanding and leveraging AI-generated insights".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/analytics-reporting/ai-insights".to_string(),
            category: "AI Features".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(22),
        },
        HelpResource {
            resource_id: "analytics_data_governance".to_string(),
            title: "Data Governance & Security".to_string(),
            description: "Best practices for data security and governance in analytics".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/analytics-reporting/data-governance".to_string(),
            category: "Security".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(28),
        },
        HelpResource {
            resource_id: "analytics_api_reference".to_string(),
            title: "Analytics API Reference".to_string(),
            description: "Complete API documentation for analytics and reporting".to_string(),
            resource_type: HelpResourceType::ApiReference,
            url: "/api/docs/analytics-reporting".to_string(),
            category: "Developer".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: None,
        },
        HelpResource {
            resource_id: "analytics_troubleshooting".to_string(),
            title: "Analytics Troubleshooting".to_string(),
            description: "Solutions to common analytics and reporting issues".to_string(),
            resource_type: HelpResourceType::Troubleshooting,
            url: "/help/analytics-reporting/troubleshooting".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(16),
        },
    ]
}
