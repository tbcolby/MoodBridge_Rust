use axum::extract::{Json as AxumJson, Query, State};
use moodbridge_rust::db;
use moodbridge_rust::handlers::{
    ai_prompt, ai_voice, dashboard_data, diff_data, health_check, DiffQuery,
};
use moodbridge_rust::models::requests::*;
use serde_json::json;
use validator::Validate;

#[tokio::test]
async fn test_health_check() {
    let response = health_check().await.unwrap();
    let json: serde_json::Value = response.0;
    assert_eq!(json["status"], "healthy");
}

#[tokio::test]
async fn test_dashboard_data() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    let response = dashboard_data(State(pool)).await.unwrap();
    // Dashboard data returns Json<Value>, so we just check it's successful
    assert!(response.0.is_object());
}

#[tokio::test]
async fn test_ai_prompt() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    let payload = json!({ "prompt": "Explain the legal term", "input_type": "text" });
    let response = ai_prompt(State(pool), AxumJson(payload)).await.unwrap();
    // AI prompt returns Json<Value>, so we just check it's successful
    assert!(response.0.is_object());
}

#[tokio::test]
async fn test_ai_voice() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    let body = axum::body::Bytes::from_static(b"test audio data");
    let response = ai_voice(State(pool), body).await.unwrap();
    // AI voice returns Json<Value>, so we just check it's successful
    assert!(response.0.is_object());
}

#[tokio::test]
async fn test_diff_data() {
    let query = Query(DiffQuery {
        file1: Some("/nonexistent1.txt".to_string()),
        file2: Some("/nonexistent2.txt".to_string()),
    });
    // This will fail because the test files don't exist, but we're testing the API structure
    let result = diff_data(query).await;
    assert!(result.is_err()); // Should fail with NOT_FOUND because test files don't exist
}

#[tokio::test]
async fn test_user_registration_validation() {
    let valid_request = UserRegistrationRequest {
        email: "valid@example.com".to_string(),
        name: "Valid User".to_string(),
        password: "ValidPassword123!".to_string(),
        password_confirm: "ValidPassword123!".to_string(),
        organization: Some("ValidOrg".to_string()),
        role: Some("admin".to_string()),
        terms_accepted: true,
        privacy_accepted: true,
    };

    assert!(valid_request.validate().is_ok());
}

#[tokio::test]
async fn test_user_registration_invalid_email() {
    let invalid_request = UserRegistrationRequest {
        email: "invalid-email".to_string(),
        name: "Invalid User".to_string(),
        password: "ValidPassword123!".to_string(),
        password_confirm: "ValidPassword123!".to_string(),
        organization: Some("SomeOrg".to_string()),
        role: Some("admin".to_string()),
        terms_accepted: true,
        privacy_accepted: true,
    };

    assert!(invalid_request.validate().is_err());
}
