// Browser Data Models Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Re-export commonly used models
pub use super::tabs::Tab;
pub use super::bookmarks::{Bookmark, BookmarkFolder};
pub use super::history::HistoryEntry;
pub use super::extensions::Extension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<Uuid>,
    pub window_state: WindowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_maximized: bool,
    pub is_fullscreen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub homepage: String,
    pub search_engine: String,
    pub language: String,
    pub theme: String,
    pub font_size: u8,
    pub enable_javascript: bool,
    pub enable_cookies: bool,
    pub enable_popups: bool,
    pub privacy_mode: bool,
    pub ad_blocking: bool,
    pub tracking_protection: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            homepage: "about:blank".to_string(),
            search_engine: "https://duckduckgo.com/?q=".to_string(),
            language: "en".to_string(),
            theme: "light".to_string(),
            font_size: 14,
            enable_javascript: true,
            enable_cookies: true,
            enable_popups: false,
            privacy_mode: false,
            ad_blocking: true,
            tracking_protection: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: Uuid,
    pub url: String,
    pub filename: String,
    pub file_path: String,
    pub file_size: u64,
    pub downloaded_bytes: u64,
    pub status: DownloadStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Completed,
    Paused,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub query: String,
    pub suggestion_type: SuggestionType,
    pub score: f32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    History,
    Bookmark,
    Search,
    TopSite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub subject: String,
    pub issuer: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub fingerprint: String,
    pub is_trusted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub id: Uuid,
    pub url: String,
    pub method: String,
    pub status_code: Option<u16>,
    pub response_time_ms: Option<u64>,
    pub request_size: u64,
    pub response_size: u64,
    pub headers: Vec<HttpHeader>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl BrowserSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            tabs: Vec::new(),
            active_tab_id: None,
            window_state: WindowState::default(),
        }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            x: 100,
            y: 100,
            is_maximized: false,
            is_fullscreen: false,
        }
    }
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            preferences: UserPreferences::default(),
            created_at: Utc::now(),
            last_login: None,
        }
    }
}

impl DownloadItem {
    pub fn new(url: String, filename: String, file_path: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            filename,
            file_path,
            file_size: 0,
            downloaded_bytes: 0,
            status: DownloadStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            mime_type: None,
        }
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.file_size == 0 {
            0.0
        } else {
            (self.downloaded_bytes as f32 / self.file_size as f32) * 100.0
        }
    }
}
