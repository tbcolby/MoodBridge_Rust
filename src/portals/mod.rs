use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::bots::BotSpecialty;
use crate::wizard::{WizardType, WizardState};

pub mod salesforce_cta_portal;
pub mod document_management_portal;
pub mod deadline_management_portal;
pub mod email_notification_portal;
pub mod analytics_reporting_portal;
pub mod portal_handlers;
pub mod portal_wizards;

/// Bot Portal Framework for managing individual bot interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotPortal {
    pub portal_id: String,
    pub bot_specialty: BotSpecialty,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color_scheme: ColorScheme,
    pub status: PortalStatus,
    pub setup_wizard: SetupWizardConfig,
    pub dashboard_config: DashboardConfig,
    pub quick_actions: Vec<QuickAction>,
    pub navigation_menu: Vec<NavigationItem>,
    pub help_resources: Vec<HelpResource>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

/// Portal status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortalStatus {
    NotConfigured,
    SetupInProgress,
    Active,
    Maintenance,
    Disabled,
    Error(String),
}

/// Color scheme for portal customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub text: String,
    pub success: String,
    pub warning: String,
    pub error: String,
}

/// Setup wizard configuration for each portal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupWizardConfig {
    pub wizard_id: String,
    pub wizard_type: WizardType,
    pub required_steps: Vec<WizardStepConfig>,
    pub estimated_time_minutes: u32,
    pub prerequisites: Vec<String>,
    pub completion_benefits: Vec<String>,
}

/// Wizard step configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStepConfig {
    pub step_id: String,
    pub title: String,
    pub description: String,
    pub step_type: WizardStepType,
    pub required: bool,
    pub estimated_minutes: u32,
    pub help_text: Option<String>,
}

/// Types of wizard steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WizardStepType {
    Configuration,
    Authentication,
    Integration,
    Testing,
    Verification,
    Training,
}

/// Dashboard configuration for each portal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub layout: DashboardLayout,
    pub widgets: Vec<DashboardWidget>,
    pub refresh_interval_seconds: u32,
    pub auto_refresh: bool,
    pub show_welcome: bool,
}

/// Dashboard layout options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DashboardLayout {
    Grid,
    Sidebar,
    Tabbed,
    Custom,
}

/// Dashboard widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub widget_id: String,
    pub title: String,
    pub widget_type: WidgetType,
    pub position: WidgetPosition,
    pub size: WidgetSize,
    pub config: serde_json::Value,
    pub visible: bool,
}

/// Types of dashboard widgets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    StatusCard,
    MetricsChart,
    RecentActivity,
    QuickStats,
    AlertsPanel,
    TaskQueue,
    PerformanceGraph,
    ConfigurationPanel,
}

/// Widget positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub row: u32,
    pub column: u32,
    pub order: u32,
}

/// Widget sizing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetSize {
    Small,   // 1x1
    Medium,  // 2x1
    Large,   // 2x2
    Wide,    // 3x1
    Tall,    // 1x3
    XLarge,  // 3x2
}

/// Quick action buttons for portal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
    pub action_id: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub action_type: ActionType,
    pub endpoint: String,
    pub requires_confirmation: bool,
    pub keyboard_shortcut: Option<String>,
}

/// Types of quick actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Execute,
    Navigate,
    Modal,
    Download,
    Upload,
    Configure,
}

/// Navigation menu items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationItem {
    pub item_id: String,
    pub label: String,
    pub icon: String,
    pub url: String,
    pub target: NavigationTarget,
    pub requires_permission: Option<String>,
    pub badge_count: Option<u32>,
    pub children: Vec<NavigationItem>,
}

/// Navigation targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavigationTarget {
    SamePage,
    NewTab,
    Modal,
    Sidebar,
}

/// Help and documentation resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpResource {
    pub resource_id: String,
    pub title: String,
    pub description: String,
    pub resource_type: HelpResourceType,
    pub url: String,
    pub category: String,
    pub difficulty_level: DifficultyLevel,
    pub estimated_read_time: Option<u32>,
}

/// Types of help resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HelpResourceType {
    Documentation,
    Tutorial,
    Video,
    FAQ,
    BestPractices,
    Troubleshooting,
    ApiReference,
}

/// Difficulty levels for help content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Portal registry for managing all bot portals
#[derive(Debug)]
pub struct PortalRegistry {
    pub portals: HashMap<String, BotPortal>,
    pub user_preferences: HashMap<String, UserPortalPreferences>,
    pub access_logs: Vec<PortalAccessLog>,
}

/// User preferences for portal customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPortalPreferences {
    pub user_id: String,
    pub favorite_portals: Vec<String>,
    pub dashboard_layout: DashboardLayout,
    pub theme: String,
    pub notifications_enabled: bool,
    pub auto_refresh: bool,
    pub default_portal: Option<String>,
    pub quick_access_items: Vec<String>,
}

/// Portal access logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalAccessLog {
    pub log_id: Uuid,
    pub user_id: String,
    pub portal_id: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: Option<u32>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Link tree structure for portal navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalLinkTree {
    pub tree_id: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<LinkCategory>,
    pub featured_portals: Vec<String>,
    pub recently_used: Vec<String>,
    pub search_enabled: bool,
}

/// Link tree categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkCategory {
    pub category_id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub portal_links: Vec<PortalLink>,
    pub expanded: bool,
}

/// Individual portal links in the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalLink {
    pub portal_id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub url: String,
    pub status_indicator: StatusIndicator,
    pub quick_setup_available: bool,
    pub estimated_setup_time: Option<u32>,
}

/// Status indicators for portal links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusIndicator {
    Ready,
    NeedsSetup,
    Active,
    Warning,
    Error,
    Maintenance,
}

impl PortalRegistry {
    pub fn new() -> Self {
        Self {
            portals: HashMap::new(),
            user_preferences: HashMap::new(),
            access_logs: Vec::new(),
        }
    }

    /// Register a new portal
    pub fn register_portal(&mut self, portal: BotPortal) {
        self.portals.insert(portal.portal_id.clone(), portal);
    }

    /// Get portal by ID
    pub fn get_portal(&self, portal_id: &str) -> Option<&BotPortal> {
        self.portals.get(portal_id)
    }

    /// Get all active portals
    pub fn get_active_portals(&self) -> Vec<&BotPortal> {
        self.portals
            .values()
            .filter(|p| matches!(p.status, PortalStatus::Active))
            .collect()
    }

    /// Log portal access
    pub fn log_access(&mut self, user_id: String, portal_id: String, action: String) {
        let log = PortalAccessLog {
            log_id: Uuid::new_v4(),
            user_id,
            portal_id,
            action,
            timestamp: Utc::now(),
            duration_seconds: None,
            success: true,
            error_message: None,
        };
        self.access_logs.push(log);
    }

    /// Generate link tree for user
    pub fn generate_link_tree(&self, user_id: Option<&str>) -> PortalLinkTree {
        let mut categories = vec![
            self.create_enterprise_category(),
            self.create_workflow_category(),
            self.create_communication_category(),
            self.create_analytics_category(),
            self.create_administration_category(),
        ];

        // Get user preferences if available
        let user_prefs = user_id.and_then(|id| self.user_preferences.get(id));
        let featured = user_prefs
            .map(|p| p.favorite_portals.clone())
            .unwrap_or_else(|| vec!["salesforce_cta".to_string(), "deadline_management".to_string()]);

        PortalLinkTree {
            tree_id: "main_portal_tree".to_string(),
            title: "MoodBridge Bot Portals".to_string(),
            description: "Access all your legal AI assistants from one central location".to_string(),
            categories,
            featured_portals: featured,
            recently_used: vec![], // Would be populated from access logs
            search_enabled: true,
        }
    }

    fn create_enterprise_category(&self) -> LinkCategory {
        LinkCategory {
            category_id: "enterprise".to_string(),
            name: "Enterprise & Integration".to_string(),
            description: "Enterprise-grade bots for large-scale operations".to_string(),
            icon: "🏢".to_string(),
            color: "#2563eb".to_string(),
            portal_links: vec![
                PortalLink {
                    portal_id: "salesforce_cta".to_string(),
                    title: "Salesforce CTA Portal".to_string(),
                    description: "Enterprise Salesforce architecture and integration guidance".to_string(),
                    icon: "⚡".to_string(),
                    url: "/portals/salesforce-cta".to_string(),
                    status_indicator: StatusIndicator::Active,
                    quick_setup_available: true,
                    estimated_setup_time: Some(15),
                },
                PortalLink {
                    portal_id: "integration_management".to_string(),
                    title: "Integration Management".to_string(),
                    description: "Manage all system integrations and API connections".to_string(),
                    icon: "🔗".to_string(),
                    url: "/portals/integration-management".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(10),
                },
            ],
            expanded: true,
        }
    }

    fn create_workflow_category(&self) -> LinkCategory {
        LinkCategory {
            category_id: "workflow".to_string(),
            name: "Workflow & Document Management".to_string(),
            description: "Streamline your legal workflows and document processes".to_string(),
            icon: "📋".to_string(),
            color: "#059669".to_string(),
            portal_links: vec![
                PortalLink {
                    portal_id: "document_management".to_string(),
                    title: "Document Management Portal".to_string(),
                    description: "Automated document processing, categorization, and version control".to_string(),
                    icon: "📄".to_string(),
                    url: "/portals/document-management".to_string(),
                    status_indicator: StatusIndicator::Active,
                    quick_setup_available: true,
                    estimated_setup_time: Some(5),
                },
                PortalLink {
                    portal_id: "deadline_management".to_string(),
                    title: "Deadline Management Portal".to_string(),
                    description: "Comprehensive deadline tracking with intelligent alerts".to_string(),
                    icon: "⏰".to_string(),
                    url: "/portals/deadline-management".to_string(),
                    status_indicator: StatusIndicator::Active,
                    quick_setup_available: true,
                    estimated_setup_time: Some(8),
                },
                PortalLink {
                    portal_id: "workflow_optimization".to_string(),
                    title: "Workflow Optimization".to_string(),
                    description: "AI-powered workflow analysis and optimization suggestions".to_string(),
                    icon: "⚙️".to_string(),
                    url: "/portals/workflow-optimization".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(12),
                },
            ],
            expanded: true,
        }
    }

    fn create_communication_category(&self) -> LinkCategory {
        LinkCategory {
            category_id: "communication".to_string(),
            name: "Communication & Client Relations".to_string(),
            description: "Automate and enhance client communication".to_string(),
            icon: "📧".to_string(),
            color: "#dc2626".to_string(),
            portal_links: vec![
                PortalLink {
                    portal_id: "email_notification".to_string(),
                    title: "Email Notification Portal".to_string(),
                    description: "Advanced email automation with templates and tracking".to_string(),
                    icon: "✉️".to_string(),
                    url: "/portals/email-notification".to_string(),
                    status_indicator: StatusIndicator::Active,
                    quick_setup_available: true,
                    estimated_setup_time: Some(7),
                },
                PortalLink {
                    portal_id: "client_communication".to_string(),
                    title: "Client Communication Hub".to_string(),
                    description: "Centralized client communication management and logging".to_string(),
                    icon: "💬".to_string(),
                    url: "/portals/client-communication".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(10),
                },
            ],
            expanded: true,
        }
    }

    fn create_analytics_category(&self) -> LinkCategory {
        LinkCategory {
            category_id: "analytics".to_string(),
            name: "Analytics & Reporting".to_string(),
            description: "Data-driven insights and comprehensive reporting".to_string(),
            icon: "📊".to_string(),
            color: "#7c3aed".to_string(),
            portal_links: vec![
                PortalLink {
                    portal_id: "analytics_reporting".to_string(),
                    title: "Analytics & Reporting Portal".to_string(),
                    description: "Comprehensive business intelligence and automated reporting".to_string(),
                    icon: "📈".to_string(),
                    url: "/portals/analytics-reporting".to_string(),
                    status_indicator: StatusIndicator::Active,
                    quick_setup_available: true,
                    estimated_setup_time: Some(15),
                },
                PortalLink {
                    portal_id: "performance_metrics".to_string(),
                    title: "Performance Metrics".to_string(),
                    description: "Track and analyze bot performance and system metrics".to_string(),
                    icon: "⚡".to_string(),
                    url: "/portals/performance-metrics".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(8),
                },
            ],
            expanded: false,
        }
    }

    fn create_administration_category(&self) -> LinkCategory {
        LinkCategory {
            category_id: "administration".to_string(),
            name: "Administration & Security".to_string(),
            description: "System administration and security management".to_string(),
            icon: "🛡️".to_string(),
            color: "#ea580c".to_string(),
            portal_links: vec![
                PortalLink {
                    portal_id: "security_monitoring".to_string(),
                    title: "Security Monitoring".to_string(),
                    description: "Advanced security monitoring and threat detection".to_string(),
                    icon: "🔒".to_string(),
                    url: "/portals/security-monitoring".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(20),
                },
                PortalLink {
                    portal_id: "user_activity_tracker".to_string(),
                    title: "User Activity Tracker".to_string(),
                    description: "Monitor user activity and system usage patterns".to_string(),
                    icon: "👥".to_string(),
                    url: "/portals/user-activity".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(12),
                },
                PortalLink {
                    portal_id: "api_management".to_string(),
                    title: "API Management".to_string(),
                    description: "Manage API endpoints, rate limits, and documentation".to_string(),
                    icon: "🔌".to_string(),
                    url: "/portals/api-management".to_string(),
                    status_indicator: StatusIndicator::NeedsSetup,
                    quick_setup_available: true,
                    estimated_setup_time: Some(15),
                },
            ],
            expanded: false,
        }
    }
}

impl Default for PortalRegistry {
    fn default() -> Self {
        Self::new()
    }
}
