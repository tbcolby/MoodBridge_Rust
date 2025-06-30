use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    Json as AxumJson,
};

use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Row;
use std::collections::HashMap;

// Re-export models
use crate::ai::{
    core_engine::{AdvancedPromptRequest, AiCoreEngine, InputType},
    AiConfig,
};
use crate::db::DbPool;
use crate::models::*;

// Health check endpoint
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "message": "Legal dashboard API is running"
    })))
}

// Dashboard HTML page - VS Code style interface
pub async fn dashboard() -> Html<String> {
    let html = std::fs::read_to_string("templates/vscode-dashboard.html").unwrap_or_else(|_| {
        std::fs::read_to_string("templates/dashboard.html")
            .unwrap_or_else(|_| "<h1>Error loading dashboard templates</h1>".to_string())
    });
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
         FROM placement_denials",
    );

    let stats_row = stats_query
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
         ORDER BY month",
    );

    let monthly_rows = monthly_query
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let monthly_trend: Vec<Value> = monthly_rows
        .iter()
        .map(|row| {
            json!({
                "month": row.get::<String, _>("month"),
                "count": row.get::<i64, _>("count")
            })
        })
        .collect();

    // Categories
    let category_query = sqlx::query(
        "SELECT 
            violation_category as category,
            COUNT(*) as count
         FROM placement_denials 
         GROUP BY violation_category
         ORDER BY count DESC",
    );

    let category_rows = category_query
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let categories: Vec<Value> = category_rows
        .iter()
        .map(|row| {
            json!({
                "category": row.get::<String, _>("category"),
                "count": row.get::<i64, _>("count")
            })
        })
        .collect();

    // Recent incidents
    let recent_query = sqlx::query(
        "SELECT denied_date, denial_reason, duration_hours
         FROM placement_denials 
         ORDER BY denied_date DESC 
         LIMIT 10",
    );

    let recent_rows = recent_query
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let recent_incidents: Vec<Value> = recent_rows
        .iter()
        .map(|row| {
            json!({
                "denied_date": row.get::<String, _>("denied_date"),
                "denial_reason": row.get::<String, _>("denial_reason"),
                "duration_hours": row.get::<f64, _>("duration_hours")
            })
        })
        .collect();

    Ok(Json(json!({
        "stats": stats,
        "monthly_trend": monthly_trend,
        "categories": categories,
        "recent_incidents": recent_incidents
    })))
}

// Advanced AI prompt endpoint with multi-modal capabilities
pub async fn ai_prompt(
    State(pool): State<DbPool>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let prompt = payload["prompt"].as_str().unwrap_or("").to_string();
    let input_type = payload["input_type"].as_str().unwrap_or("text");
    let require_citations = payload["require_citations"].as_bool().unwrap_or(false);
    let style_preference = payload["style"].as_str().map(|s| s.to_string());

    // Initialize AI Core Engine
    let ai_config = AiConfig::default();
    let ai_engine = AiCoreEngine::new(ai_config);

    // Gather current dashboard context
    let mut context = std::collections::HashMap::new();

    // Add dashboard statistics to context
    if let Ok(stats) = get_quick_stats(&pool).await {
        context.insert("current_stats".to_string(), stats);
    }

    // Add recent incidents to context
    if let Ok(recent_data) = get_recent_dashboard_data(&pool).await {
        context.insert("recent_data".to_string(), recent_data);
    }

    // Determine input type
    let parsed_input_type = match input_type {
        "voice" => InputType::Voice,
        "structured" => InputType::Structured,
        "visual" => InputType::Visual,
        "contextual" => InputType::Contextual,
        _ => InputType::Text,
    };

    // Create advanced prompt request
    let ai_request = AdvancedPromptRequest {
        input: prompt.clone(),
        input_type: parsed_input_type,
        context: Some(context),
        intent_hints: extract_intent_hints(&prompt),
        require_citations,
        max_response_length: Some(2000),
        style_preference,
    };

    // Process the request through AI Core Engine
    match ai_engine.process_advanced_prompt(ai_request).await {
        Ok(ai_response) => {
            // Convert AI response to JSON format expected by frontend
            let response_json = json!({
                "success": true,
                "primary_response": ai_response.primary_response,
                "confidence": ai_response.confidence,
                "detected_intent": ai_response.detected_intent,
                "suggested_actions": ai_response.suggested_actions,
                "contextual_insights": ai_response.contextual_insights,
                "follow_up_questions": ai_response.follow_up_questions,
                "risk_alerts": ai_response.risk_alerts,
                "citations": ai_response.citations,
                "processing_metadata": ai_response.processing_metadata,
                "action": determine_frontend_action(&ai_response.detected_intent, &ai_response.suggested_actions),
                "message": ai_response.primary_response
            });

            Ok(Json(response_json))
        }
        Err(e) => {
            // Fallback to simple responses if AI engine fails
            tracing::warn!("AI engine failed, falling back to simple responses: {}", e);
            let fallback_response = generate_fallback_response(&prompt, &pool).await?;
            Ok(Json(fallback_response))
        }
    }
}

// Real-time AI monitoring endpoint
pub async fn ai_monitor(State(pool): State<DbPool>) -> Result<Json<Value>, StatusCode> {
    let ai_config = AiConfig::default();
    let ai_engine = AiCoreEngine::new(ai_config);

    // Gather current dashboard context for monitoring
    let mut context = std::collections::HashMap::new();

    if let Ok(stats) = get_quick_stats(&pool).await {
        context.insert("current_stats".to_string(), stats);
    }

    if let Ok(recent_data) = get_recent_dashboard_data(&pool).await {
        context.insert("recent_data".to_string(), recent_data);
    }

    // Get proactive suggestions from AI
    match ai_engine.monitor_and_assist(&context).await {
        Ok(suggestions) => Ok(Json(json!({
            "success": true,
            "proactive_suggestions": suggestions,
            "monitoring_active": true,
            "timestamp": chrono::Utc::now()
        }))),
        Err(e) => {
            tracing::error!("AI monitoring failed: {}", e);
            Ok(Json(json!({
                "success": false,
                "error": "AI monitoring temporarily unavailable",
                "monitoring_active": false
            })))
        }
    }
}

// Voice processing endpoint
pub async fn ai_voice(
    State(pool): State<DbPool>,
    body: axum::body::Bytes,
) -> Result<Json<Value>, StatusCode> {
    let ai_config = AiConfig::default();
    let ai_engine = AiCoreEngine::new(ai_config);

    match ai_engine.process_voice_input(&body).await {
        Ok(ai_response) => Ok(Json(json!({
            "success": true,
            "transcription": "Audio processed",
            "response": ai_response.primary_response,
            "confidence": ai_response.confidence,
            "suggested_actions": ai_response.suggested_actions
        }))),
        Err(e) => {
            tracing::error!("Voice processing failed: {}", e);
            Ok(Json(json!({
                "success": false,
                "error": "Voice processing not available"
            })))
        }
    }
}

async fn get_quick_stats(pool: &DbPool) -> Result<Value, StatusCode> {
    let stats_query = sqlx::query(
        "SELECT 
            COUNT(*) as total_incidents,
            COALESCE(SUM(duration_hours), 0) as total_hours,
            COALESCE(AVG(duration_hours), 0) as avg_duration
         FROM placement_denials",
    );

    let stats_row = stats_query
        .fetch_one(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(json!({
        "total_incidents": stats_row.get::<i64, _>("total_incidents"),
        "total_hours": stats_row.get::<f64, _>("total_hours"),
        "avg_duration": stats_row.get::<f64, _>("avg_duration")
    }))
}

async fn get_recent_dashboard_data(pool: &DbPool) -> Result<Value, StatusCode> {
    let recent_query = sqlx::query(
        "SELECT denied_date, denial_reason, duration_hours, violation_category
         FROM placement_denials 
         ORDER BY denied_date DESC 
         LIMIT 20",
    );

    let recent_rows = recent_query
        .fetch_all(pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let recent_incidents: Vec<Value> = recent_rows
        .iter()
        .map(|row| {
            json!({
                "denied_date": row.get::<String, _>("denied_date"),
                "denial_reason": row.get::<String, _>("denial_reason"),
                "duration_hours": row.get::<f64, _>("duration_hours"),
                "violation_category": row.get::<String, _>("violation_category")
            })
        })
        .collect();

    Ok(json!({
        "recent_incidents": recent_incidents,
        "count": recent_incidents.len()
    }))
}

fn extract_intent_hints(prompt: &str) -> Vec<String> {
    let mut hints = Vec::new();
    let lower_prompt = prompt.to_lowercase();

    // Detect common intent patterns
    if lower_prompt.contains("analyze") || lower_prompt.contains("analysis") {
        hints.push("analysis_request".to_string());
    }
    if lower_prompt.contains("statistics") || lower_prompt.contains("stats") {
        hints.push("data_query".to_string());
    }
    if lower_prompt.contains("theme") || lower_prompt.contains("appearance") {
        hints.push("ui_modification".to_string());
    }
    if lower_prompt.contains("help") || lower_prompt.contains("assistance") {
        hints.push("help_request".to_string());
    }
    if lower_prompt.contains("risk") || lower_prompt.contains("danger") {
        hints.push("risk_assessment".to_string());
    }
    if lower_prompt.contains("trend") || lower_prompt.contains("pattern") {
        hints.push("pattern_detection".to_string());
    }
    if lower_prompt.contains("predict") || lower_prompt.contains("forecast") {
        hints.push("predictive_analysis".to_string());
    }

    // Default to general query if no specific patterns detected
    if hints.is_empty() {
        hints.push("general_query".to_string());
    }

    hints
}

fn determine_frontend_action(
    intent: &str,
    suggested_actions: &[crate::ai::core_engine::SuggestedAction],
) -> String {
    // Determine what action the frontend should take based on AI analysis
    match intent {
        "analysis_request" => "show_analysis_results".to_string(),
        "data_query" => "display_data".to_string(),
        "ui_modification" => "modify_interface".to_string(),
        "help_request" => "show_help_panel".to_string(),
        "risk_assessment" => "highlight_risks".to_string(),
        "pattern_detection" => "show_patterns".to_string(),
        "predictive_analysis" => "show_predictions".to_string(),
        _ => {
            // Use suggested actions to determine frontend action
            if let Some(action) = suggested_actions.first() {
                match action.action_type.as_str() {
                    "run_analysis" => "show_analysis_results".to_string(),
                    "provide_information" => "display_information".to_string(),
                    "general_assistance" => "show_message".to_string(),
                    _ => "general_response".to_string(),
                }
            } else {
                "general_response".to_string()
            }
        }
    }
}

// Diff viewer HTML page
pub async fn diff_viewer() -> Html<String> {
    let html = std::fs::read_to_string("templates/diff_viewer.html")
        .unwrap_or_else(|_| "<h1>Error loading diff viewer template</h1>".to_string());
    Html(html)
}

#[derive(Deserialize)]
pub struct DiffQuery {
    pub file1: Option<String>,
    pub file2: Option<String>,
}

// Diff data API endpoint
pub async fn diff_data(Query(params): Query<DiffQuery>) -> Result<Json<Value>, StatusCode> {
    let file1_path = params
        .file1
        .as_deref()
        .unwrap_or("/Users/tyler/Documents/mallory-legal-20250629/LegalReviewofVersion5.md");
    let file2_path = params
        .file2
        .as_deref()
        .unwrap_or("/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 5.md");

    let file1_content = match std::fs::read_to_string(file1_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!("Failed to read file1 {}: {}", file1_path, e);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let file2_content = match std::fs::read_to_string(file2_path) {
        Ok(content) => content,
        Err(e) => {
            tracing::error!("Failed to read file2 {}: {}", file2_path, e);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Calculate basic diff information
    let diff_lines = calculate_diff(&file1_content, &file2_content);

    Ok(Json(json!({
        "file1": {
            "path": file1_path,
            "content": file1_content,
            "lines": file1_content.lines().count()
        },
        "file2": {
            "path": file2_path,
            "content": file2_content,
            "lines": file2_content.lines().count()
        },
        "diff": diff_lines,
        "timestamp": chrono::Utc::now()
    })))
}

#[derive(Deserialize)]
pub struct CommitRequest {
    pub content: String,
    pub target_file: Option<String>,
}

// Commit changes endpoint
pub async fn commit_changes(
    AxumJson(payload): AxumJson<CommitRequest>,
) -> Result<Json<Value>, StatusCode> {
    let target_file = payload
        .target_file
        .as_deref()
        .unwrap_or("/Users/tyler/Documents/mallory-legal-20250629/Affidavit - Version 6.md");

    match std::fs::write(target_file, &payload.content) {
        Ok(_) => {
            tracing::info!("Successfully wrote changes to {}", target_file);

            // Try to commit to git if it's a git repository
            let git_status = std::process::Command::new("git")
                .args(&["add", target_file])
                .current_dir("/Users/tyler/Documents/mallory-legal-20250629")
                .output();

            let mut git_committed = false;
            if git_status.is_ok() {
                let git_commit = std::process::Command::new("git")
                    .args(&["commit", "-m", &format!("Update {}", target_file)])
                    .current_dir("/Users/tyler/Documents/mallory-legal-20250629")
                    .output();

                if git_commit.is_ok() {
                    git_committed = true;
                    tracing::info!("Changes committed to git repository");
                }
            }

            Ok(Json(json!({
                "success": true,
                "message": "Changes successfully saved",
                "file_path": target_file,
                "git_committed": git_committed,
                "timestamp": chrono::Utc::now()
            })))
        }
        Err(e) => {
            tracing::error!("Failed to write to {}: {}", target_file, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn calculate_diff(content1: &str, content2: &str) -> Vec<Value> {
    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    let mut diff_lines = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < lines1.len() || j < lines2.len() {
        if i >= lines1.len() {
            // Addition
            diff_lines.push(json!({
                "type": "addition",
                "line_number": j + 1,
                "content": lines2[j]
            }));
            j += 1;
        } else if j >= lines2.len() {
            // Deletion
            diff_lines.push(json!({
                "type": "deletion",
                "line_number": i + 1,
                "content": lines1[i]
            }));
            i += 1;
        } else if lines1[i] == lines2[j] {
            // Same line
            diff_lines.push(json!({
                "type": "unchanged",
                "line_number": i + 1,
                "content": lines1[i]
            }));
            i += 1;
            j += 1;
        } else {
            // Changed line
            diff_lines.push(json!({
                "type": "modification",
                "line_number": i + 1,
                "old_content": lines1[i],
                "new_content": lines2[j]
            }));
            i += 1;
            j += 1;
        }
    }

    diff_lines
}

async fn generate_fallback_response(prompt: &str, pool: &DbPool) -> Result<Value, StatusCode> {
    // Fallback to simple keyword-based responses when AI engine is unavailable
    let response = match prompt.to_lowercase().as_str() {
        p if p.contains("statistics") || p.contains("stats") => {
            json!({
                "success": true,
                "action": "show_stats",
                "message": "Here's a summary of your key statistics",
                "data": get_quick_stats(pool).await?,
                "fallback": true
            })
        }
        p if p.contains("theme") && p.contains("dark") => {
            json!({
                "success": true,
                "action": "toggle_theme",
                "target": "dark",
                "message": "Switched to dark theme",
                "fallback": true
            })
        }
        p if p.contains("theme") && p.contains("light") => {
            json!({
                "success": true,
                "action": "toggle_theme",
                "target": "light",
                "message": "Switched to light theme",
                "fallback": true
            })
        }
        p if p.contains("refresh") || p.contains("reload") => {
            json!({
                "success": true,
                "action": "refresh_data",
                "message": "Dashboard data refreshed",
                "fallback": true
            })
        }
        p if p.contains("help") => {
            json!({
                "success": true,
                "action": "show_help",
                "message": "I can help you with:\n• Changing themes\n• Refreshing data\n• Showing statistics\n• Opening settings\n• Analyzing trends",
                "fallback": true
            })
        }
        _ => {
            json!({
                "success": true,
                "action": "general_response",
                "message": format!("I understand you're asking about: '{}'. The advanced AI is temporarily unavailable, but I can help with basic commands like themes, statistics, or data refresh.", prompt),
                "fallback": true
            })
        }
    };

    Ok(response)
}
