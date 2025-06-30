use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
    limit::RequestBodyLimitLayer,
};
use tower::ServiceBuilder;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::{env, net::SocketAddr, time::Duration};
use crate::{
    config::AppConfig,
    db::{create_pool, run_migrations, seed_sample_data},
    error::AppError,
};

pub mod handlers;
pub mod models;
pub mod db;
pub mod ai;
pub mod config;
pub mod error;
pub mod middleware;
pub mod metrics;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging with better formatting
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("moodbridge_rust=info".parse().unwrap())
        )
        .with_target(false)
        .with_level(true)
        .init();

    tracing::info!("🦀⚖️ Starting MoodBridge Legal Dashboard");
    
    // Graceful startup with comprehensive error handling
    if let Err(e) = startup().await {
        tracing::error!("❌ Failed to start application: {}", e);
        std::process::exit(1);
    }
}

async fn startup() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Setup directories
    tracing::info!("📁 Setting up directories...");
    std::fs::create_dir_all("data")
        .map_err(|e| format!("Failed to create data directory: {}", e))?;
    std::fs::create_dir_all("logs")
        .map_err(|e| format!("Failed to create logs directory: {}", e))?;
    tracing::info!("✅ Directories ready");
    
    // Step 2: Database setup
    tracing::info!("🗄️  Setting up database...");
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite::memory:".into());
    tracing::info!("   Database URL: {}", database_url);
    
    let pool = create_pool(&database_url).await
        .map_err(|e| format!("Failed to create database pool: {}", e))?;
    tracing::info!("✅ Database pool created");
    
    // Step 3: Run migrations
    tracing::info!("🔄 Running database migrations...");
    run_migrations(&pool).await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;
    tracing::info!("✅ Migrations completed");
    
    // Step 4: Seed data (non-critical)
    tracing::info!("🌱 Seeding sample data...");
    if let Err(e) = seed_sample_data(&pool).await {
        tracing::warn!("⚠️  Failed to seed sample data (non-critical): {}", e);
    } else {
        tracing::info!("✅ Sample data seeded");
    }
    
    // Step 5: Build application routes
    tracing::info!("🛠️  Building application routes...");
    let app = Router::new()
        .route("/", get(handlers::dashboard))
        .route("/api/health", get(handlers::health_check))
        .route("/api/dashboard-data", get(handlers::dashboard_data))
        .route("/api/ai-prompt", axum::routing::post(handlers::ai_prompt))
        .route("/api/ai-monitor", get(handlers::ai_monitor))
        .route("/api/ai-voice", axum::routing::post(handlers::ai_voice))
        .with_state(pool.clone())
        .layer(CorsLayer::new().allow_origin(Any));
    tracing::info!("✅ Routes configured");
    
    // Step 6: Start server
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("🚀 Starting server on {}", addr);
    tracing::info!("🌐 Dashboard: http://localhost:{}", port);
    tracing::info!("📊 Health Check: http://localhost:{}/api/health", port);
    tracing::info!("🎉 MoodBridge is ready!");
    
    // Step 7: Run server with graceful shutdown
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| format!("Server error: {}", e))?;
    
    tracing::info!("👋 MoodBridge shutdown complete");
    Ok(())
}

// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("🛑 Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("🛑 Received terminate signal, shutting down gracefully...");
        },
    }
}
