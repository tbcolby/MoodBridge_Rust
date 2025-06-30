use axum::extract::{Json as AxumJson, State};
use axum::http::StatusCode;
use moodbridge_rust::db;
use moodbridge_rust::handlers::{
    ai_monitor, ai_prompt, ai_voice, dashboard_data, diff_data, health_check,
};
use moodbridge_rust::models::requests::*;
use serde_json::json;

#[tokio::test]
async fn test_health_check() {
    let response = health_check().await.unwrap();
    let json: serde_json::Value = response.0;
    assert_eq!(json["status"], "healthy");
}

#[tokio::test]
async fn test_dashboard_data() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    let response = dashboard_data(State(pool)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_ai_prompt() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    let payload = json!({ "prompt": "Explain the legal term", "input_type": "text" });
    let response = ai_prompt(State(pool), AxumJson(payload)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_ai_voice() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    let body = axum::body::Bytes::from_static(b"test audio data");
    let response = ai_voice(State(pool), body).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_diff_data() {
    let query = axum::extract::Query("file1=/test1.txt&file2=/test2.txt");
    let response = diff_data(query).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
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
