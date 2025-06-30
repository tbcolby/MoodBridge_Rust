use axum::{routing::{get, post, put}, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use std::{env, net::SocketAddr};
use crate::db::{create_pool, run_migrations, seed_sample_data};

pub mod handlers;
pub mod models;
pub mod db;
pub mod ai;
pub mod warp_command;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Read database URL from environment variable  
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let db_path = current_dir.join("data").join("main.db");
        format!("sqlite:{}", db_path.display())
    });

    // Create database pool and run migrations
    let pool = create_pool(&database_url).await.expect("Failed to create database pool");
    run_migrations(&pool).await.expect("Failed to run migrations");

    // Seed sample data
    seed_sample_data(&pool).await.expect("Failed to seed sample data");

    // Build our application with routes
    let app = Router::new()
        .route("/", get(handlers::dashboard))
        .route("/projects", get(handlers::project_dashboard))
        .route("/api/health", get(handlers::health_check))
        .route("/api/dashboard-data", get({
            let pool_clone = pool.clone();
            move || handlers::dashboard_data_simple(pool_clone.clone())
        }))
        // Project Management Routes
        .route("/api/projects", get(handlers::project::get_projects)
            .post(handlers::project::create_project))
        .route("/api/projects/:id", get(handlers::project::get_project)
            .put(handlers::project::update_project))
        .route("/api/tasks", get(handlers::project::get_tasks)
            .post(handlers::project::create_task))
        .route("/api/tasks/:id", put(handlers::project::update_task))
        .route("/api/project-dashboard", get(handlers::project::get_project_dashboard))
        .route("/api/task-analytics", get(handlers::project::get_task_analytics))
        .route("/api/work-sessions/:task_id/start", post(handlers::project::start_work_session))
        .route("/api/work-sessions/:session_id/end", put(handlers::project::end_work_session))
        .with_state(pool.clone())
        .layer(CorsLayer::new().allow_origin(Any));

    // Address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("Listening on {}", addr);

    // Run app with hyper on the configured address
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
