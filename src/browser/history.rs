// Browser History Management Module

use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub visit_count: u32,
    pub last_visit: DateTime<Utc>,
    pub first_visit: DateTime<Utc>,
    pub favicon: Option<String>,
    pub typed_count: u32,
    pub transition_type: VisitType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisitType {
    Link,
    Typed,
    Bookmark,
    AutoBookmark,
    AutoSubframe,
    ManualSubframe,
    Generated,
    StartPage,
    FormSubmit,
    Reload,
    Keyword,
    KeywordGenerated,
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub entries: Vec<HistoryEntry>,
    pub total_count: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct HistoryStats {
    pub total_visits: u32,
    pub unique_urls: u32,
    pub top_sites: Vec<TopSite>,
    pub recent_activity: Vec<DailyActivity>,
    pub most_visited_hour: u8,
    pub average_daily_visits: f32,
}

#[derive(Debug, Serialize)]
pub struct TopSite {
    pub url: String,
    pub title: String,
    pub visit_count: u32,
    pub favicon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DailyActivity {
    pub date: String,
    pub visit_count: u32,
    pub unique_sites: u32,
}

impl HistoryEntry {
    pub fn new(url: String, title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            url,
            title,
            visit_count: 1,
            last_visit: now,
            first_visit: now,
            favicon: None,
            typed_count: 0,
            transition_type: VisitType::Link,
        }
    }
}

// Get browsing history
pub async fn get_history(
    Query(params): Query<HistoryQuery>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<HistoryResponse>, AppError> {
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    
    // In a real implementation, this would query the database with filters
    let sample_entries = vec![
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "https://github.com".to_string(),
            title: "GitHub".to_string(),
            visit_count: 45,
            last_visit: Utc::now() - Duration::minutes(5),
            first_visit: Utc::now() - Duration::days(30),
            favicon: Some("📁".to_string()),
            typed_count: 12,
            transition_type: VisitType::Typed,
        },
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "https://www.rust-lang.org".to_string(),
            title: "Rust Programming Language".to_string(),
            visit_count: 28,
            last_visit: Utc::now() - Duration::hours(2),
            first_visit: Utc::now() - Duration::days(15),
            favicon: Some("🦀".to_string()),
            typed_count: 5,
            transition_type: VisitType::Bookmark,
        },
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "https://duckduckgo.com".to_string(),
            title: "DuckDuckGo — Privacy, simplified.".to_string(),
            visit_count: 67,
            last_visit: Utc::now() - Duration::hours(1),
            first_visit: Utc::now() - Duration::days(45),
            favicon: Some("🔍".to_string()),
            typed_count: 22,
            transition_type: VisitType::Typed,
        },
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "http://localhost:8080".to_string(),
            title: "MoodBridge Legal Dashboard".to_string(),
            visit_count: 156,
            last_visit: Utc::now() - Duration::minutes(2),
            first_visit: Utc::now() - Duration::days(60),
            favicon: Some("⚖️".to_string()),
            typed_count: 89,
            transition_type: VisitType::Typed,
        },
    ];

    let total_count = sample_entries.len() as u32;
    let has_more = offset + limit < total_count;

    Ok(Json(HistoryResponse {
        entries: sample_entries,
        total_count,
        has_more,
    }))
}

// Search browsing history
pub async fn search_history(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<HistoryEntry>>, AppError> {
    let query = params.get("q").unwrap_or(&String::new()).clone();
    
    // In a real implementation, this would perform full-text search on history
    let results = vec![
        HistoryEntry {
            id: Uuid::new_v4(),
            url: format!("https://example.com/search?q={}", query),
            title: format!("Search results for: {}", query),
            visit_count: 1,
            last_visit: Utc::now(),
            first_visit: Utc::now(),
            favicon: Some("🔍".to_string()),
            typed_count: 0,
            transition_type: VisitType::Link,
        },
    ];

    Ok(Json(results))
}

// Clear browsing history
pub async fn clear_history(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let timeframe = params.get("timeframe").unwrap_or(&"all".to_string()).clone();
    
    // In a real implementation, this would delete history entries based on timeframe
    let deleted_count = match timeframe.as_str() {
        "hour" => 5,
        "day" => 45,
        "week" => 250,
        "month" => 1000,
        "all" => 5000,
        _ => 0,
    };

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("History cleared for timeframe: {}", timeframe),
        "deleted_count": deleted_count
    })))
}

// Get history statistics
pub async fn get_history_stats(State(pool): State<Pool<Sqlite>>) -> Result<Json<HistoryStats>, AppError> {
    let top_sites = vec![
        TopSite {
            url: "http://localhost:8080".to_string(),
            title: "MoodBridge Dashboard".to_string(),
            visit_count: 156,
            favicon: Some("⚖️".to_string()),
        },
        TopSite {
            url: "https://github.com".to_string(),
            title: "GitHub".to_string(),
            visit_count: 89,
            favicon: Some("📁".to_string()),
        },
        TopSite {
            url: "https://duckduckgo.com".to_string(),
            title: "DuckDuckGo".to_string(),
            visit_count: 67,
            favicon: Some("🔍".to_string()),
        },
    ];

    let recent_activity = vec![
        DailyActivity {
            date: "2024-01-01".to_string(),
            visit_count: 45,
            unique_sites: 12,
        },
        DailyActivity {
            date: "2023-12-31".to_string(),
            visit_count: 38,
            unique_sites: 9,
        },
        DailyActivity {
            date: "2023-12-30".to_string(),
            visit_count: 52,
            unique_sites: 15,
        },
    ];

    Ok(Json(HistoryStats {
        total_visits: 2456,
        unique_urls: 487,
        top_sites,
        recent_activity,
        most_visited_hour: 14, // 2 PM
        average_daily_visits: 42.5,
    }))
}

// Get frequently visited sites
pub async fn get_frequent_sites(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<TopSite>>, AppError> {
    let limit: u32 = params.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    // In a real implementation, this would query most visited sites
    let frequent_sites = vec![
        TopSite {
            url: "http://localhost:8080".to_string(),
            title: "MoodBridge Dashboard".to_string(),
            visit_count: 156,
            favicon: Some("⚖️".to_string()),
        },
        TopSite {
            url: "https://github.com".to_string(),
            title: "GitHub".to_string(),
            visit_count: 89,
            favicon: Some("📁".to_string()),
        },
        TopSite {
            url: "https://www.rust-lang.org".to_string(),
            title: "Rust Programming Language".to_string(),
            visit_count: 67,
            favicon: Some("🦀".to_string()),
        },
    ];

    Ok(Json(frequent_sites))
}

// Get recently visited sites
pub async fn get_recent_sites(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<HistoryEntry>>, AppError> {
    let limit: u32 = params.get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    // In a real implementation, this would query recent visits
    let recent_sites = vec![
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "https://news.ycombinator.com".to_string(),
            title: "Hacker News".to_string(),
            visit_count: 2,
            last_visit: Utc::now() - Duration::minutes(5),
            first_visit: Utc::now() - Duration::hours(2),
            favicon: Some("📰".to_string()),
            typed_count: 1,
            transition_type: VisitType::Typed,
        },
        HistoryEntry {
            id: Uuid::new_v4(),
            url: "https://docs.rs".to_string(),
            title: "Docs.rs".to_string(),
            visit_count: 1,
            last_visit: Utc::now() - Duration::minutes(15),
            first_visit: Utc::now() - Duration::minutes(15),
            favicon: Some("📚".to_string()),
            typed_count: 0,
            transition_type: VisitType::Link,
        },
    ];

    Ok(Json(recent_sites))
}

// Delete specific history entry
pub async fn delete_history_entry(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let url = params.get("url").unwrap_or(&String::new()).clone();
    
    // In a real implementation, this would delete the specific entry
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("History entry deleted: {}", url)
    })))
}

// Export history data
pub async fn export_history(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let format = params.get("format").unwrap_or(&"json".to_string()).clone();
    
    // In a real implementation, this would generate export file
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("History exported in {} format", format),
        "download_url": "/browser/history/download/history.json",
        "entry_count": 2456
    })))
}

// Get visits for a specific URL
pub async fn get_url_visits(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let url = params.get("url").unwrap_or(&String::new()).clone();
    
    // In a real implementation, this would return visit details for the URL
    
    Ok(Json(serde_json::json!({
        "url": url,
        "total_visits": 25,
        "first_visit": "2023-11-01T10:30:00Z",
        "last_visit": "2024-01-01T15:45:00Z",
        "visits": [
            {
                "timestamp": "2024-01-01T15:45:00Z",
                "transition": "typed",
                "title": "Example Page"
            },
            {
                "timestamp": "2024-01-01T10:30:00Z",
                "transition": "link",
                "title": "Example Page"
            }
        ]
    })))
}
