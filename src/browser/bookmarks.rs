// Browser Bookmarks Management Module

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub favicon: Option<String>,
    pub folder_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub access_count: u32,
    pub is_favorite: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookmarkRequest {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub folder_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookmarkRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub folder_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BookmarksResponse {
    pub bookmarks: Vec<Bookmark>,
    pub total_count: u32,
}

impl Bookmark {
    pub fn new(title: String, url: String, description: Option<String>, folder_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            url,
            description,
            favicon: None,
            folder_id,
            tags: Vec::new(),
            created_at: Utc::now(),
            last_accessed: None,
            access_count: 0,
            is_favorite: false,
        }
    }
}

// List all bookmarks
pub async fn list_bookmarks(
    Query(params): Query<HashMap<String, String>>,
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<BookmarksResponse>, AppError> {
    // In a real implementation, this would query the database with filters
    let sample_bookmarks = vec![
        Bookmark {
            id: Uuid::new_v4(),
            title: "MoodBridge Legal Dashboard".to_string(),
            url: "http://localhost:8080".to_string(),
            description: Some("AI-powered legal case management".to_string()),
            favicon: Some("⚖️".to_string()),
            folder_id: None,
            tags: vec!["legal".to_string(), "dashboard".to_string()],
            created_at: Utc::now(),
            last_accessed: Some(Utc::now()),
            access_count: 25,
            is_favorite: true,
        },
        Bookmark {
            id: Uuid::new_v4(),
            title: "Rust Programming Language".to_string(),
            url: "https://www.rust-lang.org".to_string(),
            description: Some("Official Rust website".to_string()),
            favicon: Some("🦀".to_string()),
            folder_id: None,
            tags: vec!["programming".to_string(), "rust".to_string()],
            created_at: Utc::now(),
            last_accessed: Some(Utc::now()),
            access_count: 15,
            is_favorite: false,
        },
    ];

    Ok(Json(BookmarksResponse {
        total_count: sample_bookmarks.len() as u32,
        bookmarks: sample_bookmarks,
    }))
}

// Add a new bookmark
pub async fn add_bookmark(
    State(pool): State<Pool<Sqlite>>,
    Json(request): Json<CreateBookmarkRequest>,
) -> Result<Json<Bookmark>, AppError> {
    let mut bookmark = Bookmark::new(
        request.title,
        request.url,
        request.description,
        request.folder_id,
    );

    if let Some(tags) = request.tags {
        bookmark.tags = tags;
    }

    // In a real implementation, this would save to database

    Ok(Json(bookmark))
}

// Get a specific bookmark
pub async fn get_bookmark(
    State(pool): State<Pool<Sqlite>>,
    Path(bookmark_id): Path<Uuid>,
) -> Result<Json<Bookmark>, AppError> {
    // In a real implementation, this would query the database
    let bookmark = Bookmark {
        id: bookmark_id,
        title: "Sample Bookmark".to_string(),
        url: "https://example.com".to_string(),
        description: Some("Sample description".to_string()),
        favicon: Some("🌐".to_string()),
        folder_id: None,
        tags: vec!["sample".to_string()],
        created_at: Utc::now(),
        last_accessed: Some(Utc::now()),
        access_count: 1,
        is_favorite: false,
    };

    Ok(Json(bookmark))
}

// Update a bookmark
pub async fn update_bookmark(
    State(pool): State<Pool<Sqlite>>,
    Path(bookmark_id): Path<Uuid>,
    Json(request): Json<UpdateBookmarkRequest>,
) -> Result<Json<Bookmark>, AppError> {
    // In a real implementation, this would update the database record
    let bookmark = Bookmark {
        id: bookmark_id,
        title: request.title.unwrap_or_else(|| "Updated Bookmark".to_string()),
        url: request.url.unwrap_or_else(|| "https://example.com".to_string()),
        description: request.description,
        favicon: Some("🌐".to_string()),
        folder_id: request.folder_id,
        tags: request.tags.unwrap_or_else(Vec::new),
        created_at: Utc::now(),
        last_accessed: Some(Utc::now()),
        access_count: 1,
        is_favorite: request.is_favorite.unwrap_or(false),
    };

    Ok(Json(bookmark))
}

// Delete a bookmark
pub async fn delete_bookmark(
    State(pool): State<Pool<Sqlite>>,
    Path(bookmark_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    // In a real implementation, this would delete from database
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Bookmark {} deleted successfully", bookmark_id),
        "bookmark_id": bookmark_id
    })))
}
