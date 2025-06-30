use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html},
};

use serde_json::{json, Value};
use sqlx::Row;

// Re-export models
use crate::models::*;
use crate::db::DbPool;

// Health check endpoint
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "message": "Legal dashboard API is running"
    })))
}

// Dashboard HTML page
pub async fn dashboard() -> Html<String> {
    let html = std::fs::read_to_string("templates/dashboard.html")
        .unwrap_or_else(|_| "<h1>Error loading dashboard template</h1>".to_string());
    Html(html)
}

// Dashboard data API endpoint
pub async fn dashboard_data(State(pool): State<DbPool>) -> Result<Json<Value>, StatusCode> {
    // Get basic stats
    let stats_query = sqlx::query(
        "SELECT 
            COUNT(*) as total_incidents,
            COALESCE(SUM(duration_hours), 0) as total_hours,
            COALESCE(AVG(duration_hours), 0) as avg_duration,
            COUNT(CASE WHEN denied_date LIKE '2024-06%' THEN 1 END) as this_month
         FROM placement_denials"
    );
    
    let stats_row = stats_query.fetch_one(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let stats = json!({
        "total_incidents": stats_row.get::<i64, _>("total_incidents"),
        "total_hours": stats_row.get::<f64, _>("total_hours"),
        "avg_duration": stats_row.get::<f64, _>("avg_duration"),
        "this_month": stats_row.get::<i64, _>("this_month")
    });
    
    // Monthly trend
    let monthly_query = sqlx::query(
        "SELECT 
            substr(denied_date, 1, 7) as month,
            COUNT(*) as count
         FROM placement_denials 
         GROUP BY substr(denied_date, 1, 7)
         ORDER BY month"
    );
    
    let monthly_rows = monthly_query.fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let monthly_trend: Vec<Value> = monthly_rows.iter().map(|row| {
        json!({
            "month": row.get::<String, _>("month"),
            "count": row.get::<i64, _>("count")
        })
    }).collect();
    
    // Categories
    let category_query = sqlx::query(
        "SELECT 
            violation_category as category,
            COUNT(*) as count
         FROM placement_denials 
         GROUP BY violation_category
         ORDER BY count DESC"
    );
    
    let category_rows = category_query.fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let categories: Vec<Value> = category_rows.iter().map(|row| {
        json!({
            "category": row.get::<String, _>("category"),
            "count": row.get::<i64, _>("count")
        })
    }).collect();
    
    // Recent incidents
    let recent_query = sqlx::query(
        "SELECT denied_date, denial_reason, duration_hours
         FROM placement_denials 
         ORDER BY denied_date DESC 
         LIMIT 10"
    );
    
    let recent_rows = recent_query.fetch_all(&pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let recent_incidents: Vec<Value> = recent_rows.iter().map(|row| {
        json!({
            "denied_date": row.get::<String, _>("denied_date"),
            "denial_reason": row.get::<String, _>("denial_reason"),
            "duration_hours": row.get::<f64, _>("duration_hours")
        })
    }).collect();
    
    Ok(Json(json!({
        "stats": stats,
        "monthly_trend": monthly_trend,
        "categories": categories,
        "recent_incidents": recent_incidents
    })))
}

// AI prompt endpoint for enhanced interactions
pub async fn ai_prompt(
    State(pool): State<DbPool>,
    Json(payload): Json<Value>
) -> Result<Json<Value>, StatusCode> {
    let prompt = payload["prompt"].as_str().unwrap_or("");
    
    // Simple keyword-based AI response (replace with actual AI integration)
    let response = match prompt.to_lowercase().as_str() {
        p if p.contains("statistics") || p.contains("stats") => {
            json!({
                "action": "show_stats",
                "message": "Here's a summary of your key statistics",
                "data": get_quick_stats(&pool).await?
            })
        },
        p if p.contains("theme") && p.contains("dark") => {
            json!({
                "action": "toggle_theme",
                "target": "dark",
                "message": "Switched to dark theme"
            })
        },
        p if p.contains("theme") && p.contains("light") => {
            json!({
                "action": "toggle_theme",
                "target": "light",
                "message": "Switched to light theme"
            })
        },
        p if p.contains("refresh") || p.contains("reload") => {
            json!({
                "action": "refresh_data",
                "message": "Dashboard data refreshed"
            })
        },
        p if p.contains("help") => {
            json!({
                "action": "show_help",
                "message": "I can help you with:\n• Changing themes\n• Refreshing data\n• Showing statistics\n• Opening settings\n• Analyzing trends"
            })
        },
        _ => {
            json!({
                "action": "general_response",
                "message": format!("I understand you're asking about: '{}'. Try asking about themes, statistics, or data refresh.", prompt)
            })
        }
    };
    
    Ok(Json(response))
}

async fn get_quick_stats(pool: &DbPool) -> Result<Value, StatusCode> {
    let stats_query = sqlx::query(
        "SELECT 
            COUNT(*) as total_incidents,
            COALESCE(SUM(duration_hours), 0) as total_hours,
            COALESCE(AVG(duration_hours), 0) as avg_duration
         FROM placement_denials"
    );
    
    let stats_row = stats_query.fetch_one(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(json!({
        "total_incidents": stats_row.get::<i64, _>("total_incidents"),
        "total_hours": stats_row.get::<f64, _>("total_hours"),
        "avg_duration": stats_row.get::<f64, _>("avg_duration")
    }))
}
