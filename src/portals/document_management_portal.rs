use super::*;
use crate::bots::BotSpecialty;
use crate::wizard::WizardType;
use chrono::Utc;
use serde_json::json;

/// Create the Document Management Portal configuration
pub fn create_document_management_portal() -> BotPortal {
    BotPortal {
        portal_id: "document_management".to_string(),
        bot_specialty: BotSpecialty::DocumentManagement,
        name: "Document Management Portal".to_string(),
        description: "Comprehensive document lifecycle management with automated processing, version control, and intelligent categorization. Streamline document workflows from creation to archival.".to_string(),
        icon: "📄".to_string(),
        color_scheme: ColorScheme {
            primary: "#2563eb".to_string(),    // Blue
            secondary: "#1e40af".to_string(),   // Dark Blue
            accent: "#f59e0b".to_string(),      // Amber
            background: "#f8fafc".to_string(),  // Light Gray
            text: "#1e293b".to_string(),        // Slate
            success: "#10b981".to_string(),     // Emerald
            warning: "#f59e0b".to_string(),     // Amber
            error: "#ef4444".to_string(),       // Red
        },
        status: PortalStatus::Active,
        setup_wizard: create_document_setup_wizard(),
        dashboard_config: create_document_dashboard(),
        quick_actions: create_document_quick_actions(),
        navigation_menu: create_document_navigation(),
        help_resources: create_document_help_resources(),
        created_at: Utc::now(),
        last_accessed: None,
    }
}

fn create_document_setup_wizard() -> SetupWizardConfig {
    SetupWizardConfig {
        wizard_id: "document_management_setup".to_string(),
        wizard_type: WizardType::DocumentManagement,
        required_steps: vec![
            WizardStepConfig {
                step_id: "doc_storage_config".to_string(),
                title: "Configure Document Storage".to_string(),
                description: "Set up document repositories and storage locations".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 8,
                help_text: Some("Configure local, cloud, and network storage options".to_string()),
            },
            WizardStepConfig {
                step_id: "doc_categories_setup".to_string(),
                title: "Document Categories & Tags".to_string(),
                description: "Define document categories, tags, and classification rules".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 12,
                help_text: Some("Create taxonomy for organizing legal documents".to_string()),
            },
            WizardStepConfig {
                step_id: "doc_workflow_config".to_string(),
                title: "Workflow Configuration".to_string(),
                description: "Set up document approval workflows and routing rules".to_string(),
                step_type: WizardStepType::Configuration,
                required: true,
                estimated_minutes: 15,
                help_text: Some("Define how documents flow through review and approval processes".to_string()),
            },
            WizardStepConfig {
                step_id: "doc_access_permissions".to_string(),
                title: "Access Control & Permissions".to_string(),
                description: "Configure user roles and document access permissions".to_string(),
                step_type: WizardStepType::Security,
                required: true,
                estimated_minutes: 10,
                help_text: Some("Set up role-based access control for document security".to_string()),
            },
            WizardStepConfig {
                step_id: "doc_automation_rules".to_string(),
                title: "Automation Rules".to_string(),
                description: "Configure automated processing and notification rules".to_string(),
                step_type: WizardStepType::Configuration,
                required: false,
                estimated_minutes: 8,
                help_text: Some("Set up rules for automatic document processing and alerts".to_string()),
            },
        ],
        estimated_time_minutes: 53,
        prerequisites: vec![
            "Administrative access to document storage systems".to_string(),
            "List of document types and categories".to_string(),
            "User roles and permissions matrix".to_string(),
        ],
        completion_benefits: vec![
            "Automated document categorization and filing".to_string(),
            "Streamlined document approval workflows".to_string(),
            "Enhanced document security and access control".to_string(),
            "Intelligent document search and retrieval".to_string(),
            "Automated version control and audit trails".to_string(),
        ],
    }
}

fn create_document_dashboard() -> DashboardConfig {
    DashboardConfig {
        layout: DashboardLayout::Grid,
        widgets: vec![
            DashboardWidget {
                widget_id: "doc_storage_overview".to_string(),
                title: "Storage Overview".to_string(),
                widget_type: WidgetType::StatusCard,
                position: WidgetPosition { row: 1, column: 1, order: 1 },
                size: WidgetSize::Medium,
                config: json!({
                    "show_usage": true,
                    "show_capacity": true,
                    "show_trends": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "doc_processing_queue".to_string(),
                title: "Processing Queue".to_string(),
                widget_type: WidgetType::TaskQueue,
                position: WidgetPosition { row: 1, column: 2, order: 2 },
                size: WidgetSize::Large,
                config: json!({
                    "show_progress": true,
                    "auto_refresh": true,
                    "max_items": 10
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "doc_categories_stats".to_string(),
                title: "Document Categories".to_string(),
                widget_type: WidgetType::DonutChart,
                position: WidgetPosition { row: 2, column: 1, order: 3 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "donut",
                    "show_percentages": true,
                    "interactive": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "doc_recent_activity".to_string(),
                title: "Recent Document Activity".to_string(),
                widget_type: WidgetType::RecentActivity,
                position: WidgetPosition { row: 2, column: 2, order: 4 },
                size: WidgetSize::Medium,
                config: json!({
                    "max_items": 8,
                    "show_user": true,
                    "show_timestamps": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "doc_workflow_status".to_string(),
                title: "Workflow Status".to_string(),
                widget_type: WidgetType::AlertsPanel,
                position: WidgetPosition { row: 3, column: 1, order: 5 },
                size: WidgetSize::Wide,
                config: json!({
                    "show_pending": true,
                    "show_overdue": true,
                    "group_by_priority": true
                }),
                visible: true,
            },
            DashboardWidget {
                widget_id: "doc_search_analytics".to_string(),
                title: "Search Analytics".to_string(),
                widget_type: WidgetType::MetricsChart,
                position: WidgetPosition { row: 4, column: 1, order: 6 },
                size: WidgetSize::Medium,
                config: json!({
                    "chart_type": "line",
                    "time_range": "7_days",
                    "metrics": ["searches", "downloads", "views"]
                }),
                visible: true,
            },
        ],
        refresh_interval_seconds: 30,
        auto_refresh: true,
        show_welcome: true,
    }
}

fn create_document_quick_actions() -> Vec<QuickAction> {
    vec![
        QuickAction {
            action_id: "doc_upload_bulk".to_string(),
            label: "Bulk Upload".to_string(),
            description: "Upload multiple documents with automatic categorization".to_string(),
            icon: "📤".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/documents/bulk-upload".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+U".to_string()),
        },
        QuickAction {
            action_id: "doc_search_advanced".to_string(),
            label: "Advanced Search".to_string(),
            description: "Perform advanced document search with filters and metadata".to_string(),
            icon: "🔍".to_string(),
            action_type: ActionType::Modal,
            endpoint: "/api/documents/search".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+F".to_string()),
        },
        QuickAction {
            action_id: "doc_workflow_create".to_string(),
            label: "Create Workflow".to_string(),
            description: "Design new document approval or processing workflow".to_string(),
            icon: "⚙️".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/document-management/workflows/new".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+W".to_string()),
        },
        QuickAction {
            action_id: "doc_audit_trail".to_string(),
            label: "Audit Trail".to_string(),
            description: "View comprehensive audit trail and document history".to_string(),
            icon: "📋".to_string(),
            action_type: ActionType::Navigate,
            endpoint: "/portals/document-management/audit".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+A".to_string()),
        },
        QuickAction {
            action_id: "doc_export_report".to_string(),
            label: "Export Report".to_string(),
            description: "Generate and export document management reports".to_string(),
            icon: "📊".to_string(),
            action_type: ActionType::Download,
            endpoint: "/api/documents/reports/export".to_string(),
            requires_confirmation: false,
            keyboard_shortcut: Some("Ctrl+E".to_string()),
        },
        QuickAction {
            action_id: "doc_cleanup_wizard".to_string(),
            label: "Cleanup Wizard".to_string(),
            description: "Run automated cleanup for duplicate and obsolete documents".to_string(),
            icon: "🧹".to_string(),
            action_type: ActionType::Execute,
            endpoint: "/api/documents/cleanup".to_string(),
            requires_confirmation: true,
            keyboard_shortcut: Some("Ctrl+L".to_string()),
        },
    ]
}

fn create_document_navigation() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            item_id: "doc_dashboard".to_string(),
            label: "Dashboard".to_string(),
            icon: "📊".to_string(),
            url: "/portals/document-management".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "doc_library".to_string(),
            label: "Document Library".to_string(),
            icon: "📚".to_string(),
            url: "/portals/document-management/library".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![
                NavigationItem {
                    item_id: "doc_browse".to_string(),
                    label: "Browse Documents".to_string(),
                    icon: "📂".to_string(),
                    url: "/portals/document-management/library/browse".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "doc_categories".to_string(),
                    label: "Categories".to_string(),
                    icon: "🏷️".to_string(),
                    url: "/portals/document-management/library/categories".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "doc_workflows".to_string(),
            label: "Workflows".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/document-management/workflows".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: Some(5), // Active workflows
            children: vec![
                NavigationItem {
                    item_id: "doc_workflow_active".to_string(),
                    label: "Active Workflows".to_string(),
                    icon: "🔄".to_string(),
                    url: "/portals/document-management/workflows/active".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
                NavigationItem {
                    item_id: "doc_workflow_templates".to_string(),
                    label: "Templates".to_string(),
                    icon: "📋".to_string(),
                    url: "/portals/document-management/workflows/templates".to_string(),
                    target: NavigationTarget::SamePage,
                    requires_permission: None,
                    badge_count: None,
                    children: vec![],
                },
            ],
        },
        NavigationItem {
            item_id: "doc_analytics".to_string(),
            label: "Analytics".to_string(),
            icon: "📈".to_string(),
            url: "/portals/document-management/analytics".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: None,
            badge_count: None,
            children: vec![],
        },
        NavigationItem {
            item_id: "doc_settings".to_string(),
            label: "Settings".to_string(),
            icon: "⚙️".to_string(),
            url: "/portals/document-management/settings".to_string(),
            target: NavigationTarget::SamePage,
            requires_permission: Some("admin".to_string()),
            badge_count: None,
            children: vec![],
        },
    ]
}

fn create_document_help_resources() -> Vec<HelpResource> {
    vec![
        HelpResource {
            resource_id: "doc_getting_started".to_string(),
            title: "Getting Started with Document Management".to_string(),
            description: "Complete guide to setting up and using the document management system".to_string(),
            resource_type: HelpResourceType::Tutorial,
            url: "/help/document-management/getting-started".to_string(),
            category: "Setup".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(10),
        },
        HelpResource {
            resource_id: "doc_workflow_guide".to_string(),
            title: "Document Workflow Design Guide".to_string(),
            description: "Best practices for designing efficient document workflows".to_string(),
            resource_type: HelpResourceType::BestPractices,
            url: "/help/document-management/workflows".to_string(),
            category: "Workflows".to_string(),
            difficulty_level: DifficultyLevel::Intermediate,
            estimated_read_time: Some(20),
        },
        HelpResource {
            resource_id: "doc_security_guide".to_string(),
            title: "Document Security & Access Control".to_string(),
            description: "Comprehensive guide to securing documents and managing access".to_string(),
            resource_type: HelpResourceType::Documentation,
            url: "/help/document-management/security".to_string(),
            category: "Security".to_string(),
            difficulty_level: DifficultyLevel::Advanced,
            estimated_read_time: Some(25),
        },
        HelpResource {
            resource_id: "doc_api_reference".to_string(),
            title: "Document Management API Reference".to_string(),
            description: "Complete API documentation for developers and integrators".to_string(),
            resource_type: HelpResourceType::ApiReference,
            url: "/api/docs/document-management".to_string(),
            category: "Developer".to_string(),
            difficulty_level: DifficultyLevel::Expert,
            estimated_read_time: None,
        },
        HelpResource {
            resource_id: "doc_troubleshooting".to_string(),
            title: "Troubleshooting Common Issues".to_string(),
            description: "Solutions to frequently encountered problems".to_string(),
            resource_type: HelpResourceType::Troubleshooting,
            url: "/help/document-management/troubleshooting".to_string(),
            category: "Support".to_string(),
            difficulty_level: DifficultyLevel::Beginner,
            estimated_read_time: Some(15),
        },
    ]
}
