use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::*;
use super::salesforce_cta_portal::create_salesforce_cta_portal;
use crate::wizard::{WizardManager, CreateWizardRequest};

/// Portal application state
#[derive(Debug)]
pub struct PortalAppState {
    pub portal_registry: Arc<RwLock<PortalRegistry>>,
    pub wizard_manager: Arc<RwLock<WizardManager>>,
}

/// Query parameters for portal listing
#[derive(Debug, Deserialize)]
pub struct PortalListQuery {
    pub category: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}

/// Portal setup request
#[derive(Debug, Deserialize)]
pub struct PortalSetupRequest {
    pub portal_id: String,
    pub config_overrides: Option<serde_json::Value>,
    pub skip_wizard: Option<bool>,
}

/// Portal action request
#[derive(Debug, Deserialize)]
pub struct PortalActionRequest {
    pub action_id: String,
    pub parameters: Option<serde_json::Value>,
}

/// Portal action response
#[derive(Debug, Serialize)]
pub struct PortalActionResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub message: String,
    pub next_actions: Vec<String>,
}

/// Create portal router with all endpoints
pub fn create_portal_router() -> Router<PortalAppState> {
    Router::new()
        // Portal Discovery & Navigation
        .route("/", get(portal_home))
        .route("/link-tree", get(get_link_tree))
        .route("/portal/:portal_id", get(get_portal_page))
        .route("/api/portals", get(list_portals))
        .route("/api/portals/:portal_id", get(get_portal_config))
        
        // Portal Management
        .route("/api/portals/:portal_id/setup", post(setup_portal))
        .route("/api/portals/:portal_id/action", post(execute_portal_action))
        .route("/api/portals/:portal_id/status", get(get_portal_status))
        
        // Dashboard & Widgets
        .route("/api/portals/:portal_id/dashboard", get(get_portal_dashboard))
        .route("/api/portals/:portal_id/widgets/:widget_id", get(get_widget_data))
        
        // Portal-specific routes
        .route("/portals/salesforce-cta", get(salesforce_cta_portal_page))
        .route("/portals/salesforce-cta/*path", get(salesforce_cta_subpage))
        .route("/portals/document-management", get(document_management_portal_page))
        .route("/portals/deadline-management", get(deadline_management_portal_page))
        .route("/portals/email-notification", get(email_notification_portal_page))
        .route("/portals/analytics-reporting", get(analytics_reporting_portal_page))
}

/// Portal home page with link tree
pub async fn portal_home(State(state): State<PortalAppState>) -> Html<String> {
    let registry = state.portal_registry.read().await;
    let link_tree = registry.generate_link_tree(None);
    
    let html = generate_portal_home_html(&link_tree);
    Html(html)
}

/// Get link tree as JSON
pub async fn get_link_tree(
    State(state): State<PortalAppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<PortalLinkTree> {
    let registry = state.portal_registry.read().await;
    let user_id = params.get("user_id").map(|s| s.as_str());
    let link_tree = registry.generate_link_tree(user_id);
    
    Json(link_tree)
}

/// Get portal page HTML
pub async fn get_portal_page(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
) -> Result<Html<String>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    match registry.get_portal(&portal_id) {
        Some(portal) => {
            let html = generate_portal_html(portal);
            Ok(Html(html))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// List all portals with filtering
pub async fn list_portals(
    State(state): State<PortalAppState>,
    Query(query): Query<PortalListQuery>,
) -> Json<Vec<BotPortal>> {
    let registry = state.portal_registry.read().await;
    let mut portals: Vec<_> = registry.portals.values().cloned().collect();
    
    // Apply filters
    if let Some(status) = &query.status {
        portals.retain(|p| format!("{:?}", p.status).to_lowercase().contains(&status.to_lowercase()));
    }
    
    if let Some(search) = &query.search {
        let search_lower = search.to_lowercase();
        portals.retain(|p| {
            p.name.to_lowercase().contains(&search_lower) ||
            p.description.to_lowercase().contains(&search_lower)
        });
    }
    
    Json(portals)
}

/// Get portal configuration
pub async fn get_portal_config(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
) -> Result<Json<BotPortal>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    match registry.get_portal(&portal_id) {
        Some(portal) => Ok(Json(portal.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Setup a portal (run wizard if needed)
pub async fn setup_portal(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
    Json(request): Json<PortalSetupRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    let portal = registry.get_portal(&portal_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    if request.skip_wizard.unwrap_or(false) {
        // Skip wizard, direct setup
        Ok(Json(serde_json::json!({
            "status": "configured",
            "message": "Portal configured successfully",
            "portal_url": format!("/portals/{}", portal_id)
        })))
    } else {
        // Create wizard instance
        let mut wizard_manager = state.wizard_manager.write().await;
        let wizard_request = CreateWizardRequest {
            wizard_type: portal.setup_wizard.wizard_type.clone(),
            initial_data: Some(HashMap::from([
                ("portal_id".to_string(), serde_json::Value::String(portal_id.clone())),
            ])),
        };
        
        match wizard_manager.create_wizard(wizard_request).await {
            Ok(wizard_state) => {
                Ok(Json(serde_json::json!({
                    "status": "wizard_created",
                    "wizard_id": wizard_state.id,
                    "wizard_url": format!("/wizards/{}", wizard_state.id),
                    "estimated_time": portal.setup_wizard.estimated_time_minutes
                })))
            }
            Err(e) => {
                eprintln!("Error creating wizard: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Execute a portal action
pub async fn execute_portal_action(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
    Json(request): Json<PortalActionRequest>,
) -> Result<Json<PortalActionResponse>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    let portal = registry.get_portal(&portal_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Find the action
    let action = portal.quick_actions.iter()
        .find(|a| a.action_id == request.action_id)
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Simulate action execution (in real implementation, this would call the actual bot)
    let response = match action.action_type {
        ActionType::Execute => {
            PortalActionResponse {
                success: true,
                result: Some(serde_json::json!({
                    "action": action.action_id,
                    "status": "completed",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
                message: format!("Successfully executed: {}", action.label),
                next_actions: vec!["view_results".to_string(), "generate_report".to_string()],
            }
        }
        ActionType::Navigate => {
            PortalActionResponse {
                success: true,
                result: Some(serde_json::json!({
                    "redirect_url": action.endpoint
                })),
                message: format!("Navigating to: {}", action.label),
                next_actions: vec![],
            }
        }
        ActionType::Download => {
            PortalActionResponse {
                success: true,
                result: Some(serde_json::json!({
                    "download_url": format!("{}/download", action.endpoint),
                    "filename": format!("{}_report.pdf", portal_id)
                })),
                message: format!("Preparing download: {}", action.label),
                next_actions: vec!["open_file".to_string()],
            }
        }
        _ => {
            PortalActionResponse {
                success: true,
                result: None,
                message: format!("Action queued: {}", action.label),
                next_actions: vec![],
            }
        }
    };
    
    Ok(Json(response))
}

/// Get portal status
pub async fn get_portal_status(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    let portal = registry.get_portal(&portal_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(serde_json::json!({
        "portal_id": portal_id,
        "status": portal.status,
        "last_accessed": portal.last_accessed,
        "health_score": 85.5,
        "active_tasks": 3,
        "recent_activity": [
            {"action": "architecture_review", "timestamp": "2024-01-15T10:30:00Z"},
            {"action": "performance_analysis", "timestamp": "2024-01-15T09:15:00Z"}
        ]
    })))
}

/// Get portal dashboard data
pub async fn get_portal_dashboard(
    Path(portal_id): Path<String>,
    State(state): State<PortalAppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    let portal = registry.get_portal(&portal_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Generate mock dashboard data based on portal configuration
    let dashboard_data = generate_dashboard_data(&portal.dashboard_config);
    
    Ok(Json(dashboard_data))
}

/// Get widget data
pub async fn get_widget_data(
    Path((portal_id, widget_id)): Path<(String, String)>,
    State(state): State<PortalAppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let registry = state.portal_registry.read().await;
    
    let portal = registry.get_portal(&portal_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Find the widget
    let widget = portal.dashboard_config.widgets.iter()
        .find(|w| w.widget_id == widget_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    // Generate mock widget data
    let widget_data = generate_widget_data(widget);
    
    Ok(Json(widget_data))
}

/// Salesforce CTA Portal page
pub async fn salesforce_cta_portal_page(
    State(state): State<PortalAppState>,
) -> Html<String> {
    let portal = create_salesforce_cta_portal();
    let html = generate_portal_html(&portal);
    Html(html)
}

/// Salesforce CTA subpage handler
pub async fn salesforce_cta_subpage(
    Path(path): Path<String>,
    State(state): State<PortalAppState>,
) -> Html<String> {
    let portal = create_salesforce_cta_portal();
    let html = generate_salesforce_subpage_html(&portal, &path);
    Html(html)
}

/// Document Management Portal page
pub async fn document_management_portal_page(
    State(state): State<PortalAppState>,
) -> Html<String> {
    let html = generate_document_management_html();
    Html(html)
}

/// Deadline Management Portal page
pub async fn deadline_management_portal_page(
    State(state): State<PortalAppState>,
) -> Html<String> {
    let html = generate_deadline_management_html();
    Html(html)
}

/// Email Notification Portal page
pub async fn email_notification_portal_page(
    State(state): State<PortalAppState>,
) -> Html<String> {
    let html = generate_email_notification_html();
    Html(html)
}

/// Analytics Reporting Portal page
pub async fn analytics_reporting_portal_page(
    State(state): State<PortalAppState>,
) -> Html<String> {
    let html = generate_analytics_reporting_html();
    Html(html)
}

/// Generate portal home HTML with link tree
fn generate_portal_home_html(link_tree: &PortalLinkTree) -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}
        .header {{
            text-align: center;
            color: white;
            margin-bottom: 3rem;
        }}
        .header h1 {{
            font-size: 3rem;
            margin: 0;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }}
        .header p {{
            font-size: 1.2rem;
            opacity: 0.9;
            margin-top: 1rem;
        }}
        .search-bar {{
            max-width: 600px;
            margin: 0 auto 3rem;
            position: relative;
        }}
        .search-bar input {{
            width: 100%;
            padding: 1rem 1.5rem;
            font-size: 1.1rem;
            border: none;
            border-radius: 50px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
        }}
        .categories {{
            display: grid;
            gap: 2rem;
        }}
        .category {{
            background: white;
            border-radius: 20px;
            padding: 2rem;
            box-shadow: 0 8px 32px rgba(0,0,0,0.1);
            transition: transform 0.3s ease;
        }}
        .category:hover {{
            transform: translateY(-5px);
        }}
        .category-header {{
            display: flex;
            align-items: center;
            margin-bottom: 1.5rem;
        }}
        .category-icon {{
            font-size: 2rem;
            margin-right: 1rem;
        }}
        .category-title {{
            font-size: 1.5rem;
            font-weight: 600;
            margin: 0;
        }}
        .category-description {{
            color: #666;
            margin-bottom: 1.5rem;
        }}
        .portal-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 1.5rem;
        }}
        .portal-card {{
            padding: 1.5rem;
            border: 2px solid #f0f0f0;
            border-radius: 15px;
            transition: all 0.3s ease;
            cursor: pointer;
            position: relative;
        }}
        .portal-card:hover {{
            border-color: #667eea;
            transform: translateY(-2px);
            box-shadow: 0 4px 20px rgba(0,0,0,0.1);
        }}
        .portal-icon {{
            font-size: 2rem;
            margin-bottom: 1rem;
        }}
        .portal-title {{
            font-size: 1.2rem;
            font-weight: 600;
            margin-bottom: 0.5rem;
        }}
        .portal-description {{
            color: #666;
            font-size: 0.9rem;
            line-height: 1.4;
        }}
        .status-indicator {{
            position: absolute;
            top: 1rem;
            right: 1rem;
            width: 12px;
            height: 12px;
            border-radius: 50%;
        }}
        .status-active {{ background: #22c55e; }}
        .status-needs-setup {{ background: #f59e0b; }}
        .status-warning {{ background: #ef4444; }}
        .setup-time {{
            margin-top: 0.5rem;
            font-size: 0.8rem;
            color: #888;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>{}</h1>
            <p>{}</p>
        </div>
        
        <div class="search-bar">
            <input type="text" placeholder="Search for a bot portal..." onkeyup="filterPortals(this.value)">
        </div>
        
        <div class="categories">
            {}
        </div>
    </div>
    
    <script>
        function filterPortals(searchTerm) {{
            // Implementation for search filtering
            console.log('Searching for:', searchTerm);
        }}
        
        function openPortal(url) {{
            window.location.href = url;
        }}
    </script>
</body>
</html>
    "#, 
    link_tree.title,
    link_tree.title,
    link_tree.description,
    generate_categories_html(&link_tree.categories)
    )
}

/// Generate categories HTML
fn generate_categories_html(categories: &[LinkCategory]) -> String {
    categories.iter().map(|category| {
        format!(r#"
            <div class="category">
                <div class="category-header">
                    <div class="category-icon" style="color: {}">{}</div>
                    <h2 class="category-title">{}</h2>
                </div>
                <p class="category-description">{}</p>
                <div class="portal-grid">
                    {}
                </div>
            </div>
        "#, 
        category.color,
        category.icon,
        category.name,
        category.description,
        generate_portal_cards_html(&category.portal_links)
        )
    }).collect::<Vec<_>>().join("")
}

/// Generate portal cards HTML
fn generate_portal_cards_html(portals: &[PortalLink]) -> String {
    portals.iter().map(|portal| {
        let status_class = match portal.status_indicator {
            StatusIndicator::Active => "status-active",
            StatusIndicator::NeedsSetup => "status-needs-setup",
            StatusIndicator::Warning | StatusIndicator::Error => "status-warning",
            _ => "status-active",
        };
        
        let setup_time = if let Some(time) = portal.estimated_setup_time {
            format!("<div class='setup-time'>⏱️ Setup: {} min</div>", time)
        } else {
            String::new()
        };
        
        format!(r#"
            <div class="portal-card" onclick="openPortal('{}')">
                <div class="status-indicator {}"></div>
                <div class="portal-icon">{}</div>
                <div class="portal-title">{}</div>
                <div class="portal-description">{}</div>
                {}
            </div>
        "#, 
        portal.url,
        status_class,
        portal.icon,
        portal.title,
        portal.description,
        setup_time
        )
    }).collect::<Vec<_>>().join("")
}

/// Generate individual portal HTML
fn generate_portal_html(portal: &BotPortal) -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background-color: {};
            color: {};
        }}
        .portal-header {{
            background: linear-gradient(135deg, {} 0%, {} 100%);
            color: white;
            padding: 2rem 0;
            text-align: center;
        }}
        .portal-nav {{
            background: white;
            border-bottom: 1px solid #e5e7eb;
            padding: 1rem 0;
        }}
        .nav-container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem;
        }}
        .nav-links {{
            display: flex;
            gap: 2rem;
            list-style: none;
            margin: 0;
            padding: 0;
        }}
        .nav-link {{
            text-decoration: none;
            color: {};
            font-weight: 500;
            padding: 0.5rem 1rem;
            border-radius: 8px;
            transition: background-color 0.3s;
        }}
        .nav-link:hover {{
            background-color: {};
        }}
        .main-content {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}
        .dashboard {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 2rem;
        }}
        .widget {{
            background: white;
            border-radius: 12px;
            padding: 1.5rem;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        .quick-actions {{
            display: flex;
            gap: 1rem;
            margin-bottom: 2rem;
            flex-wrap: wrap;
        }}
        .action-btn {{
            background: {};
            color: white;
            border: none;
            padding: 0.75rem 1.5rem;
            border-radius: 8px;
            cursor: pointer;
            font-weight: 500;
            transition: all 0.3s;
        }}
        .action-btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.2);
        }}
    </style>
</head>
<body>
    <div class="portal-header">
        <h1>{} {}</h1>
        <p>{}</p>
    </div>
    
    <nav class="portal-nav">
        <div class="nav-container">
            <ul class="nav-links">
                {}
            </ul>
        </div>
    </nav>
    
    <main class="main-content">
        <div class="quick-actions">
            {}
        </div>
        
        <div class="dashboard">
            {}
        </div>
    </main>
    
    <script>
        function executeAction(actionId) {{
            console.log('Executing action:', actionId);
            // Implementation for action execution
        }}
    </script>
</body>
</html>
    "#,
    portal.name,
    portal.color_scheme.background,
    portal.color_scheme.text,
    portal.color_scheme.primary,
    portal.color_scheme.secondary,
    portal.color_scheme.text,
    format!("{}20", portal.color_scheme.primary), // 20% opacity
    portal.color_scheme.primary,
    portal.icon,
    portal.name,
    portal.description,
    generate_nav_links_html(&portal.navigation_menu),
    generate_quick_actions_html(&portal.quick_actions),
    generate_dashboard_widgets_html(&portal.dashboard_config.widgets)
    )
}

/// Generate navigation links HTML
fn generate_nav_links_html(nav_items: &[NavigationItem]) -> String {
    nav_items.iter().map(|item| {
        format!(r#"<li><a href="{}" class="nav-link">{} {}</a></li>"#, 
            item.url, item.icon, item.label)
    }).collect::<Vec<_>>().join("")
}

/// Generate quick actions HTML
fn generate_quick_actions_html(actions: &[QuickAction]) -> String {
    actions.iter().map(|action| {
        format!(r#"<button class="action-btn" onclick="executeAction('{}')" title="{}">{} {}</button>"#,
            action.action_id, action.description, action.icon, action.label)
    }).collect::<Vec<_>>().join("")
}

/// Generate dashboard widgets HTML
fn generate_dashboard_widgets_html(widgets: &[DashboardWidget]) -> String {
    widgets.iter().filter(|w| w.visible).map(|widget| {
        format!(r#"
            <div class="widget">
                <h3>{}</h3>
                <div id="widget-{}" class="widget-content">
                    Loading {}...
                </div>
            </div>
        "#, widget.title, widget.widget_id, widget.title)
    }).collect::<Vec<_>>().join("")
}

// Subpage generators for specific portals
fn generate_salesforce_subpage_html(portal: &BotPortal, path: &str) -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
</head>
<body>
    <h1>Salesforce CTA - {}</h1>
    <p>This is the {} section of the Salesforce CTA Portal.</p>
    <a href="/portals/salesforce-cta">← Back to Dashboard</a>
</body>
</html>
    "#, portal.name, path, path, path)
}

fn generate_document_management_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document Management Portal</title>
</head>
<body>
    <h1>📄 Document Management Portal</h1>
    <p>Automated document processing, categorization, and version control.</p>
    <a href="/">← Back to Portal Hub</a>
</body>
</html>
    "#.to_string()
}

fn generate_deadline_management_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Deadline Management Portal</title>
</head>
<body>
    <h1>⏰ Deadline Management Portal</h1>
    <p>Comprehensive deadline tracking with intelligent alerts and crisis management.</p>
    <a href="/">← Back to Portal Hub</a>
</body>
</html>
    "#.to_string()
}

fn generate_email_notification_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Email Notification Portal</title>
</head>
<body>
    <h1>📧 Email Notification Portal</h1>
    <p>Advanced email automation with templates, tracking, and analytics.</p>
    <a href="/">← Back to Portal Hub</a>
</body>
</html>
    "#.to_string()
}

fn generate_analytics_reporting_html() -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Analytics & Reporting Portal</title>
</head>
<body>
    <h1>📊 Analytics & Reporting Portal</h1>
    <p>Comprehensive business intelligence and automated reporting.</p>
    <a href="/">← Back to Portal Hub</a>
</body>
</html>
    "#.to_string()
}

/// Generate mock dashboard data
fn generate_dashboard_data(config: &DashboardConfig) -> serde_json::Value {
    serde_json::json!({
        "layout": config.layout,
        "refresh_interval": config.refresh_interval_seconds,
        "widgets": config.widgets.iter().map(|w| {
            serde_json::json!({
                "widget_id": w.widget_id,
                "title": w.title,
                "type": w.widget_type,
                "data": generate_widget_data(w)
            })
        }).collect::<Vec<_>>()
    })
}

/// Generate mock widget data
fn generate_widget_data(widget: &DashboardWidget) -> serde_json::Value {
    match widget.widget_type {
        WidgetType::StatusCard => serde_json::json!({
            "status": "healthy",
            "score": 85.5,
            "trend": "up",
            "last_updated": chrono::Utc::now().to_rfc3339()
        }),
        WidgetType::MetricsChart => serde_json::json!({
            "chart_type": "line",
            "data_points": [
                {"timestamp": "2024-01-01T00:00:00Z", "value": 120},
                {"timestamp": "2024-01-02T00:00:00Z", "value": 135},
                {"timestamp": "2024-01-03T00:00:00Z", "value": 110}
            ]
        }),
        WidgetType::RecentActivity => serde_json::json!({
            "activities": [
                {"action": "Architecture Review", "timestamp": "2024-01-15T10:30:00Z", "status": "completed"},
                {"action": "Performance Analysis", "timestamp": "2024-01-15T09:15:00Z", "status": "in_progress"}
            ]
        }),
        WidgetType::QuickStats => serde_json::json!({
            "stats": [
                {"label": "Active Projects", "value": 12, "change": "+2"},
                {"label": "Completion Rate", "value": "94%", "change": "+5%"}
            ]
        }),
        _ => serde_json::json!({
            "message": "Widget data loading...",
            "widget_id": widget.widget_id
        })
    }
}
