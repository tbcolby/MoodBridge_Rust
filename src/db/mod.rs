use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::Path;
use tracing::{error, info};

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Connecting to database: {}", database_url);
    
    // Ensure the database file exists
    if !Path::new(database_url.strip_prefix("sqlite://").unwrap_or(database_url)).exists() {
        info!("Database file doesn't exist, will be created");
    }
    
    let pool = SqlitePool::connect(database_url).await?;
    
    info!("Database connection established");
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    info!("Running database migrations");
    
    // Read the schema file and execute it
    let schema = include_str!("../../data/schema.sql");
    
    sqlx::query(schema).execute(pool).await?;
    
    info!("Database schema initialized");
    Ok(())
}

pub async fn seed_sample_data(pool: &DbPool) -> Result<(), sqlx::Error> {
    info!("Seeding sample placement denial data");
    
    let sample_denials = vec![
        ("2025-02-11", "09:00", "15:00", 6.0, "Refusal without cause", "Stipulation Violation", "Text messages"),
        ("2025-02-14", "16:00", "20:00", 4.0, "Last minute cancellation", "Placement Interference", "Email chain"),
        ("2025-02-18", "10:00", "18:00", 8.0, "Claimed illness", "Pattern Behavior", "Medical excuse"),
        ("2025-02-25", "14:00", "19:00", 5.0, "Transportation issues", "Communication Gap", "Phone log"),
        ("2025-03-05", "09:00", "17:00", 8.0, "Schedule conflict", "Placement Denial", "Calendar evidence"),
        ("2025-03-12", "11:00", "16:00", 5.0, "Child resistance claim", "Alienation Tactic", "Witness statement"),
        ("2025-03-19", "15:00", "20:00", 5.0, "Activity conflict", "Prioritization Issue", "Activity schedule"),
        ("2025-03-26", "09:00", "12:00", 3.0, "Weather excuse", "Minimal Effort", "Weather report"),
        ("2025-04-02", "13:00", "18:00", 5.0, "Vehicle problems", "Repeated Excuse", "Maintenance record"),
        ("2025-04-09", "10:00", "16:00", 6.0, "Medical appointment", "Scheduling Conflict", "Appointment slip"),
        ("2025-04-16", "14:00", "19:00", 5.0, "Extended family visit", "Priority Override", "Family photos"),
        ("2025-04-23", "09:00", "15:00", 6.0, "School event", "Activity Prioritization", "School notice"),
        ("2025-04-30", "16:00", "20:00", 4.0, "Sudden plans", "Last Minute Change", "Text exchange"),
        ("2025-05-07", "11:00", "17:00", 6.0, "Mother's Day prep", "Holiday Excuse", "Shopping receipts"),
        ("2025-05-14", "09:00", "14:00", 5.0, "Friend's birthday", "Social Priority", "Party invitation"),
        ("2025-05-21", "15:00", "19:00", 4.0, "Tired child", "Physical Excuse", "Bedtime log"),
        ("2025-05-28", "10:00", "16:00", 6.0, "Memorial Day plans", "Holiday Override", "Family gathering"),
        ("2025-06-04", "13:00", "18:00", 5.0, "Swimming lessons", "Activity Conflict", "Lesson schedule"),
        ("2025-06-11", "09:00", "15:00", 6.0, "Graduation party", "Event Priority", "Graduation invite"),
        ("2025-06-14", "08:00", "20:00", 12.0, "Father's Day denied", "Holiday Violation", "Text refusal"),
        ("2025-06-18", "16:00", "19:00", 3.0, "Dinner plans", "Social Override", "Restaurant receipt"),
        ("2025-06-25", "11:00", "17:00", 6.0, "Camp preparation", "Summer Activity", "Camp materials"),
    ];
    
    // Check if data already exists
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM placement_denials")
        .fetch_one(pool)
        .await?;
    
    if count.0 > 0 {
        info!("Sample data already exists, skipping");
        return Ok(());
    }
    
    for denial in sample_denials {
        sqlx::query(
            "INSERT INTO placement_denials 
             (denied_date, requested_start_time, requested_end_time, duration_hours, 
              denial_reason, violation_category, evidence_attached) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(denial.0)
        .bind(denial.1)
        .bind(denial.2)
        .bind(denial.3)
        .bind(denial.4)
        .bind(denial.5)
        .bind(denial.6)
        .execute(pool)
        .await?;
    }
    
    // Add timeline events
    let timeline_events = vec![
        ("2018-10-12", "court", "Divorce Judgment Entered", "Initial custody arrangements established", 5),
        ("2023-12-05", "agreement", "Provider-First Stipulation Filed", "Stipulation establishing placement schedule", 4),
        ("2025-02-11", "violation", "First Placement Denial", "Beginning of systematic placement interference", 5),
        ("2025-02-12", "mediation", "Mediator Directive Issued", "Formal directive regarding placement compliance", 4),
        ("2025-06-14", "violation", "Father's Day Denied", "Complete denial of Father's Day placement", 5),
        ("2025-06-26", "court", "Motion to Enforce Filed", "Legal action initiated for enforcement", 5),
    ];
    
    for event in timeline_events {
        sqlx::query(
            "INSERT INTO timeline_events 
             (event_date, event_type, event_title, event_description, importance_level) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(event.0)
        .bind(event.1)
        .bind(event.2)
        .bind(event.3)
        .bind(event.4)
        .execute(pool)
        .await?;
    }
    
    // Add violations
    let violations = vec![
        ("2025-02-11", "Placement Interference", "Systematic denial pattern begins", "Stipulation Section 3.2", 4),
        ("2025-06-14", "Holiday Violation", "Father's Day completely denied", "Stipulation Section 2.1", 5),
        ("2025-06-26", "Contempt Pattern", "Ongoing non-compliance with court orders", "Multiple violations", 5),
    ];
    
    for violation in violations {
        sqlx::query(
            "INSERT INTO violations 
             (violation_date, violation_type, description, stipulation_reference, impact_score) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(violation.0)
        .bind(violation.1)
        .bind(violation.2)
        .bind(violation.3)
        .bind(violation.4)
        .execute(pool)
        .await?;
    }
    
    info!("Sample data seeded successfully");
    Ok(())
}
