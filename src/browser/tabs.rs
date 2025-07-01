// Browser Tabs Management Module

use axum::{
    extract::{Path, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub is_active: bool,
    pub is_loading: bool,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub memory_usage_kb: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTabRequest {
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TabsResponse {
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<Uuid>,
    pub total_memory_usage_kb: u32,
}

impl Tab {
    pub fn new(url: Option<String>, title: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.unwrap_or_else(|| "New Tab".to_string()),
            url: url.unwrap_or_else(|| "about:blank".to_string()),
            favicon: None,
            is_active: false,
            is_loading: false,
            created_at: now,
            last_accessed: now,
            memory_usage_kb: 1024, // Base memory usage
        }
    }
}

// List all tabs
pub async fn list_tabs(State(pool): State<Pool<Sqlite>>) -> Result<Json<TabsResponse>, AppError> {
    // In a real implementation, this would query the database
    let sample_tabs = vec![
        Tab {
            id: Uuid::new_v4(),
            title: "MoodBridge Dashboard".to_string(),
            url: "http://localhost:8080".to_string(),
            favicon: Some("🏠".to_string()),
            is_active: true,
            is_loading: false,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            memory_usage_kb: 2048,
        },
        Tab {
            id: Uuid::new_v4(),
            title: "DuckDuckGo Search".to_string(),
            url: "https://duckduckgo.com".to_string(),
            favicon: Some("🔍".to_string()),
            is_active: false,
            is_loading: false,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            memory_usage_kb: 1536,
        },
    ];

    let total_memory = sample_tabs.iter().map(|t| t.memory_usage_kb).sum();
    let active_tab_id = sample_tabs.iter().find(|t| t.is_active).map(|t| t.id);

    Ok(Json(TabsResponse {
        tabs: sample_tabs,
        active_tab_id,
        total_memory_usage_kb: total_memory,
    }))
}

// Create a new tab
pub async fn create_tab(
    State(pool): State<Pool<Sqlite>>,
    Json(request): Json<CreateTabRequest>,
) -> Result<Json<Tab>, AppError> {
    let tab = Tab::new(request.url, request.title);
    
    // In a real implementation, this would save to database
    
    Ok(Json(tab))
}

// Close a tab
pub async fn close_tab(
    State(pool): State<Pool<Sqlite>>,
    Path(tab_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    // In a real implementation, this would delete from database
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Tab {} closed successfully", tab_id),
        "tab_id": tab_id
    })))
}

// Activate a tab
pub async fn activate_tab(
    State(pool): State<Pool<Sqlite>>,
    Path(tab_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    // In a real implementation, this would update the database
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Tab {} activated", tab_id),
        "active_tab_id": tab_id
    })))
}
