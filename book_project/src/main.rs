use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{info, warn};
use uuid::Uuid;

mod agents;
mod handlers;
mod models;
mod templates;
mod wizard;

use agents::{Agent, TeamCoordinator};
use handlers::*;
use models::*;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Sqlite>,
    team_coordinator: Arc<TeamCoordinator>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Setup database
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:book_writer.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Initialize team coordinator with AI agents
    let team_coordinator = Arc::new(TeamCoordinator::new());
    
    // Register AI agents
    let lead_writer = Box::new(agents::lead_writer::LeadWriterBot::new());
    team_coordinator.register_agent(lead_writer).await?;
    
    let app_state = AppState {
        db: pool,
        team_coordinator,
    };

    // Build the application routes
    let app = Router::new()
        // Main application routes
        .route("/", get(home_handler))
        .route("/wizard", get(wizard_start))
        .route("/wizard/:step", get(wizard_step).post(wizard_step_submit))
        
        // Book project management
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id/chapters", get(list_chapters).post(create_chapter))
        .route("/projects/:id/chapters/:chapter_id", get(get_chapter))
        
        // AI agent management
        .route("/agents", get(list_agents))
        .route("/agents/:id/tasks", post(assign_task_to_agent))
        
        // Writing interface
        .route("/write/:project_id", get(writing_interface))
        .route("/preview/:project_id", get(preview_book))
        
        // API endpoints
        .route("/api/generate-content", post(generate_content_api))
        .route("/api/agent-collaboration", post(agent_collaboration_api))
        
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    info!("📚 MoodBridge Book Writer starting on http://localhost:3001");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Handler implementations will be in separate modules
async fn home_handler() -> impl IntoResponse {
    Html(templates::HOME_TEMPLATE)
}

async fn wizard_start() -> impl IntoResponse {
    Html(templates::WIZARD_START_TEMPLATE)
}

async fn wizard_step(Path(step): Path<String>) -> impl IntoResponse {
    match step.as_str() {
        "book-type" => Html(templates::WIZARD_BOOK_TYPE_TEMPLATE),
        "writing-style" => Html(templates::WIZARD_WRITING_STYLE_TEMPLATE),
        "agents" => Html(templates::WIZARD_AGENTS_TEMPLATE),
        "collaboration" => Html(templates::WIZARD_COLLABORATION_TEMPLATE),
        "review" => Html(templates::WIZARD_REVIEW_TEMPLATE),
        _ => (StatusCode::NOT_FOUND, "Step not found").into_response(),
    }
}

#[derive(Deserialize)]
struct WizardStepData {
    step: String,
    data: HashMap<String, serde_json::Value>,
}

async fn wizard_step_submit(
    Path(step): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<WizardStepData>,
) -> impl IntoResponse {
    // Process wizard step data and move to next step
    match wizard::process_step(&step, payload.data).await {
        Ok(next_step) => {
            Json(serde_json::json!({
                "success": true,
                "next_step": next_step,
                "message": "Step completed successfully"
            }))
        }
        Err(e) => {
            warn!("Wizard step error: {}", e);
            Json(serde_json::json!({
                "success": false,
                "error": e.to_string()
            }))
        }
    }
}

async fn list_projects(State(state): State<AppState>) -> impl IntoResponse {
    // Implementation for listing book projects
    Json(serde_json::json!({
        "projects": []
    }))
}

async fn create_project(
    State(state): State<AppState>,
    Json(project): Json<BookProject>,
) -> impl IntoResponse {
    // Implementation for creating new book project
    Json(serde_json::json!({
        "success": true,
        "project_id": Uuid::new_v4()
    }))
}

async fn get_project(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Implementation for getting specific project
    Json(serde_json::json!({
        "project": {
            "id": id,
            "title": "The MoodBridge Chronicles",
            "status": "in_progress"
        }
    }))
}

async fn list_chapters(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "chapters": []
    }))
}

async fn create_chapter(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(chapter): Json<Chapter>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "chapter_id": Uuid::new_v4()
    }))
}

async fn get_chapter(
    Path((project_id, chapter_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "chapter": {
            "id": chapter_id,
            "title": "Chapter 1: The Strange Loop of Legal Reasoning",
            "content": "Chapter content would be loaded here..."
        }
    }))
}

async fn list_agents(State(state): State<AppState>) -> impl IntoResponse {
    Json(serde_json::json!({
        "agents": [
            {
                "id": Uuid::new_v4(),
                "name": "Lead Writer Bot",
                "type": "HofstadterianWriter",
                "status": "active",
                "capabilities": ["philosophical_analysis", "narrative_structure", "technical_writing"]
            }
        ]
    }))
}

async fn assign_task_to_agent(
    Path(agent_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(task): Json<agents::Task>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "task_id": task.id
    }))
}

async fn writing_interface(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Html(templates::WRITING_INTERFACE_TEMPLATE)
}

async fn preview_book(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    Html(templates::BOOK_PREVIEW_TEMPLATE)
}

async fn generate_content_api(
    State(state): State<AppState>,
    Json(request): Json<ContentGenerationRequest>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "content": "Generated content would appear here...",
        "word_count": 1500,
        "style_analysis": {
            "hofstadterian_score": 0.85,
            "philosophical_depth": 0.92,
            "technical_accuracy": 0.88
        }
    }))
}

async fn agent_collaboration_api(
    State(state): State<AppState>,
    Json(message): Json<agents::CollaborationMessage>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "response": "Collaboration message processed"
    }))
}
