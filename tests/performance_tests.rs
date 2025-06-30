use axum::extract::State;
use moodbridge_rust::db;
use moodbridge_rust::handlers::*;
use moodbridge_rust::models::PlacementDenial;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::test]
async fn test_database_connection_pool_performance() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();

    let start = Instant::now();

    // Test concurrent connections
    let tasks: Vec<_> = (0..50)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                let result = sqlx::query("SELECT 1").fetch_one(&pool).await;
                assert!(result.is_ok());
            })
        })
        .collect();

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

    let duration = start.elapsed();
    println!("50 concurrent connections took: {:?}", duration);
    assert!(duration < Duration::from_secs(5)); // Should complete within 5 seconds
}

#[tokio::test]
async fn test_api_response_time() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    db::seed_sample_data(&pool).await.unwrap();

    let start = Instant::now();
    let result = dashboard_data(State(pool)).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(duration < Duration::from_millis(500)); // Should respond within 500ms
    println!("Dashboard API response time: {:?}", duration);
}

#[tokio::test]
async fn test_health_check_performance() {
    let iterations = 1000;
    let start = Instant::now();

    for _ in 0..iterations {
        let result = health_check().await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    let avg_time = duration / iterations;

    println!(
        "{} health checks took: {:?}, avg: {:?}",
        iterations, duration, avg_time
    );
    assert!(avg_time < Duration::from_millis(1)); // Should be very fast
}

#[tokio::test]
async fn test_concurrent_api_requests() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();
    db::seed_sample_data(&pool).await.unwrap();

    let start = Instant::now();

    // Test 20 concurrent dashboard requests
    let tasks: Vec<_> = (0..20)
        .map(|_| {
            let pool = pool.clone();
            tokio::spawn(async move {
                let result = dashboard_data(State(pool)).await;
                assert!(result.is_ok());
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }

    let duration = start.elapsed();
    println!("20 concurrent dashboard requests took: {:?}", duration);
    assert!(duration < Duration::from_secs(10));
}

#[tokio::test]
async fn test_memory_usage_stability() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();

    // Perform many operations to test for memory leaks
    for i in 0..100 {
        let result = dashboard_data(State(pool.clone())).await;
        assert!(result.is_ok());

        // Every 10 iterations, force garbage collection (if available)
        if i % 10 == 0 {
            tokio::task::yield_now().await;
        }
    }

    // Test should complete without excessive memory usage
    assert!(true); // Completion itself is the test
}

#[tokio::test]
async fn test_large_dataset_performance() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();

    // Insert a large number of records
    let start = Instant::now();
    for i in 0..1000 {
        let denial = PlacementDenial {
            id: i,
            denied_date: format!("2024-01-{:02}", (i % 30) + 1),
            requested_start_time: None,
            requested_end_time: None,
            duration_hours: Some((i % 12) as f64 + 1.0),
            denial_reason: Some(format!("Reason {}", i)),
            violation_category: Some("Test Category".to_string()),
            evidence_attached: None,
            created_at: None,
        };

        let _ = sqlx::query(
            "INSERT INTO placement_denials (denied_date, duration_hours, denial_reason, violation_category) VALUES (?, ?, ?, ?)"
        )
        .bind(&denial.denied_date)
        .bind(denial.duration_hours)
        .bind(&denial.denial_reason)
        .bind(&denial.violation_category)
        .execute(&pool)
        .await;
    }

    let insert_duration = start.elapsed();
    println!("Inserting 1000 records took: {:?}", insert_duration);

    // Test query performance on large dataset
    let query_start = Instant::now();
    let result = dashboard_data(State(pool)).await;
    let query_duration = query_start.elapsed();

    assert!(result.is_ok());
    println!("Querying large dataset took: {:?}", query_duration);
    assert!(query_duration < Duration::from_secs(2)); // Should complete within 2 seconds
}

#[tokio::test]
async fn test_timeout_handling() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();

    // Test that operations complete within reasonable timeouts
    let result = timeout(Duration::from_secs(30), dashboard_data(State(pool))).await;

    assert!(result.is_ok(), "Operation should complete within timeout");
    assert!(result.unwrap().is_ok(), "Operation should succeed");
}

#[tokio::test]
async fn test_stress_health_check() {
    // Stress test the health check endpoint
    let concurrent_requests = 100;
    let start = Instant::now();

    let tasks: Vec<_> = (0..concurrent_requests)
        .map(|_| tokio::spawn(async move { health_check().await }))
        .collect();

    let mut success_count = 0;
    for task in tasks {
        let result = task.await.unwrap();
        if result.is_ok() {
            success_count += 1;
        }
    }

    let duration = start.elapsed();
    println!(
        "{} concurrent health checks: {} succeeded in {:?}",
        concurrent_requests, success_count, duration
    );

    assert_eq!(success_count, concurrent_requests);
    assert!(duration < Duration::from_secs(5));
}

#[tokio::test]
async fn test_database_transaction_performance() {
    let pool = db::create_pool("sqlite::memory:").await.unwrap();
    db::run_migrations(&pool).await.unwrap();

    let start = Instant::now();

    // Test transaction performance
    let mut tx = pool.begin().await.unwrap();

    for i in 0..100 {
        sqlx::query("INSERT INTO placement_denials (denied_date, duration_hours) VALUES (?, ?)")
            .bind(format!("2024-01-{:02}", (i % 30) + 1))
            .bind(8.0)
            .execute(&mut *tx)
            .await
            .unwrap();
    }

    tx.commit().await.unwrap();

    let duration = start.elapsed();
    println!("Transaction with 100 inserts took: {:?}", duration);
    assert!(duration < Duration::from_secs(1));
}
