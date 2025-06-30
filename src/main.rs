use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use std::{env, net::SocketAddr};
use crate::db::{create_pool, run_migrations, seed_sample_data};

pub mod handlers;
pub mod models;
pub mod db;
pub mod ai;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Read database URL from environment variable
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/main.db".into());

    // Create database pool and run migrations
    let pool = create_pool(&database_url).await.expect("Failed to create database pool");
    run_migrations(&pool).await.expect("Failed to run migrations");

    // Seed sample data
    seed_sample_data(&pool).await.expect("Failed to seed sample data");

    // Build our application with routes
    let app = Router::new()
        .route("/", get(handlers::dashboard))
        .route("/api/health", get(handlers::health_check))
        .route("/api/dashboard-data", get({
            let pool_clone = pool.clone();
            move || handlers::dashboard_data_simple(pool_clone.clone())
        }))
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
