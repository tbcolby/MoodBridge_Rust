use axum::http::StatusCode;
use moodbridge_rust::handlers::{commit_changes, diff_data, diff_viewer, CommitRequest, DiffQuery};
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_diff_viewer_html_response() {
    let response = diff_viewer().await;
    let content = response.0;

    // Verify HTML structure
    assert!(content.contains("<!DOCTYPE html>"));
    assert!(content.contains("MoodBridge Diff Viewer"));
    assert!(content.to_lowercase().contains("monaco"));
}

#[tokio::test]
async fn test_diff_data_with_test_files() {
    // Create temporary test files
    let temp_dir = tempdir().unwrap();

    let file1_path = temp_dir.path().join("test1.md");
    let file2_path = temp_dir.path().join("test2.md");

    fs::write(&file1_path, "Line 1\nLine 2\nLine 3").unwrap();
    fs::write(&file2_path, "Line 1\nModified Line 2\nLine 3\nNew Line 4").unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    // Test diff_data function
    let result = diff_data(axum::extract::Query(query)).await;
    assert!(result.is_ok());

    let json_response = result.unwrap().0;

    // Verify response structure
    assert!(json_response["file1"]["content"].is_string());
    assert!(json_response["file2"]["content"].is_string());
    assert!(json_response["diff"].is_array());
    assert!(json_response["timestamp"].is_string());

    // Verify file content
    assert_eq!(json_response["file1"]["lines"], 3);
    assert_eq!(json_response["file2"]["lines"], 4);
}

#[tokio::test]
async fn test_diff_data_with_nonexistent_files() {
    let query = DiffQuery {
        file1: Some("/nonexistent/file1.md".to_string()),
        file2: Some("/nonexistent/file2.md".to_string()),
    };

    let result = diff_data(axum::extract::Query(query)).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_commit_changes_with_temp_file() {
    let temp_dir = tempdir().unwrap();
    let target_file = temp_dir.path().join("output.md");

    let request = CommitRequest {
        content: "# Test Document\n\nThis is test content.".to_string(),
        target_file: Some(target_file.to_string_lossy().to_string()),
    };

    let result = commit_changes(axum::Json(request)).await;
    assert!(result.is_ok());

    let json_response = result.unwrap().0;

    // Verify response
    assert_eq!(json_response["success"], true);
    assert!(json_response["message"].is_string());
    assert!(json_response["file_path"].is_string());
    assert!(json_response["timestamp"].is_string());

    // Verify file was created
    assert!(target_file.exists());
    let content = fs::read_to_string(&target_file).unwrap();
    assert_eq!(content, "# Test Document\n\nThis is test content.");
}

#[tokio::test]
async fn test_commit_changes_with_invalid_path() {
    let request = CommitRequest {
        content: "Test content".to_string(),
        target_file: Some("/invalid/path/that/does/not/exist.md".to_string()),
    };

    let result = commit_changes(axum::Json(request)).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_diff_algorithm_edge_cases() {
    let temp_dir = tempdir().unwrap();

    // Test with identical files
    let file1_path = temp_dir.path().join("identical1.md");
    let file2_path = temp_dir.path().join("identical2.md");

    let content = "Same content\nIn both files\nExactly identical";
    fs::write(&file1_path, content).unwrap();
    fs::write(&file2_path, content).unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    let result = diff_data(axum::extract::Query(query)).await.unwrap();
    let diff_array = result.0["diff"].as_array().unwrap();

    // All lines should be "unchanged"
    for diff_item in diff_array {
        assert_eq!(diff_item["type"], "unchanged");
    }
}

#[tokio::test]
async fn test_diff_algorithm_additions_and_deletions() {
    let temp_dir = tempdir().unwrap();

    let file1_path = temp_dir.path().join("original.md");
    let file2_path = temp_dir.path().join("modified.md");

    fs::write(&file1_path, "Line 1\nLine 2\nLine 3\nLine 4").unwrap();
    fs::write(&file2_path, "Line 1\nNew Line 2\nLine 4\nLine 5").unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    let result = diff_data(axum::extract::Query(query)).await.unwrap();
    let diff_array = result.0["diff"].as_array().unwrap();

    // Verify we have different types of changes
    let diff_types: Vec<&str> = diff_array
        .iter()
        .map(|item| item["type"].as_str().unwrap())
        .collect();

    assert!(diff_types.contains(&"unchanged"));
    assert!(diff_types.contains(&"addition") || diff_types.contains(&"modification"));
}

#[tokio::test]
async fn test_empty_files() {
    let temp_dir = tempdir().unwrap();

    let file1_path = temp_dir.path().join("empty1.md");
    let file2_path = temp_dir.path().join("empty2.md");

    fs::write(&file1_path, "").unwrap();
    fs::write(&file2_path, "").unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    let result = diff_data(axum::extract::Query(query)).await.unwrap();

    assert_eq!(result.0["file1"]["lines"], 0);
    assert_eq!(result.0["file2"]["lines"], 0);
    assert_eq!(result.0["diff"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_large_file_handling() {
    let temp_dir = tempdir().unwrap();

    let file1_path = temp_dir.path().join("large1.md");
    let file2_path = temp_dir.path().join("large2.md");

    // Create files with 1000 lines each
    let mut content1 = String::new();
    let mut content2 = String::new();

    for i in 1..=1000 {
        content1.push_str(&format!("Line {} in file 1\n", i));
        if i == 500 {
            content2.push_str("Modified line 500\n");
        } else {
            content2.push_str(&format!("Line {} in file 1\n", i));
        }
    }

    fs::write(&file1_path, content1).unwrap();
    fs::write(&file2_path, content2).unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    let start = std::time::Instant::now();
    let result = diff_data(axum::extract::Query(query)).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration.as_secs() < 5); // Should complete within 5 seconds

    let response = result.unwrap().0;
    assert_eq!(response["file1"]["lines"], 1000);
    assert_eq!(response["file2"]["lines"], 1000);
}

#[tokio::test]
async fn test_unicode_content() {
    let temp_dir = tempdir().unwrap();

    let file1_path = temp_dir.path().join("unicode1.md");
    let file2_path = temp_dir.path().join("unicode2.md");

    let content1 = "Hello 世界\nÄÖÜ ñoño\n🚀 Emoji test";
    let content2 = "Hello 世界\nÄÖÜ ñoño modified\n🚀 Emoji test\n新行";

    fs::write(&file1_path, content1).unwrap();
    fs::write(&file2_path, content2).unwrap();

    let query = DiffQuery {
        file1: Some(file1_path.to_string_lossy().to_string()),
        file2: Some(file2_path.to_string_lossy().to_string()),
    };

    let result = diff_data(axum::extract::Query(query)).await;
    assert!(result.is_ok());

    let response = result.unwrap().0;
    let file1_content = response["file1"]["content"].as_str().unwrap();
    let file2_content = response["file2"]["content"].as_str().unwrap();

    assert!(file1_content.contains("世界"));
    assert!(file2_content.contains("新行"));
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_concurrent_diff_operations() {
        let temp_dir = tempdir().unwrap();

        // Create test files
        let mut handles = Vec::new();

        for i in 0..10 {
            let temp_dir = temp_dir.path().to_path_buf();
            let handle = tokio::spawn(async move {
                let file1_path = temp_dir.join(format!("test1_{}.md", i));
                let file2_path = temp_dir.join(format!("test2_{}.md", i));

                fs::write(
                    &file1_path,
                    format!("Content {} line 1\nContent {} line 2", i, i),
                )
                .unwrap();
                fs::write(
                    &file2_path,
                    format!("Content {} line 1\nModified {} line 2", i, i),
                )
                .unwrap();

                let query = DiffQuery {
                    file1: Some(file1_path.to_string_lossy().to_string()),
                    file2: Some(file2_path.to_string_lossy().to_string()),
                };

                let start = Instant::now();
                let result = diff_data(axum::extract::Query(query)).await;
                let duration = start.elapsed();

                (result.is_ok(), duration)
            });

            handles.push(handle);
        }

        let results = futures::future::join_all(handles).await;

        for result in results {
            let (success, duration) = result.unwrap();
            assert!(success);
            assert!(duration.as_millis() < 1000); // Each operation should complete quickly
        }
    }
}
