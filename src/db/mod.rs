use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::Path;
use tracing::{error, info};

pub type DbPool = Pool<Sqlite>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    info!("Connecting to database: {}", database_url);
    
    // Ensure the database directory exists
    let db_path = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"))
        .unwrap_or(database_url);
    
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }
    if !Path::new(db_path).exists() {
        info!("Database file doesn't exist, will be created at: {}", db_path);
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
    info!("Seeding sample placement incident data");
    
    let sample_denials = vec![
        // January 2024
        ("2024-01-08", "14:00", "19:00", 5.0, "Winter activity conflict", "Scheduling Issue", "Activity schedule"),
        ("2024-01-15", "09:00", "17:00", 8.0, "MLK Day plans", "Holiday Priority", "Family event notice"),
        ("2024-01-22", "16:00", "20:00", 4.0, "Tutoring session", "Educational Priority", "Tutor confirmation"),
        ("2024-01-29", "10:00", "15:00", 5.0, "Medical checkup", "Health Priority", "Appointment card"),
        
        // February 2024  
        ("2024-02-05", "13:00", "18:00", 5.0, "Birthday party", "Social Priority", "Party invitation"),
        ("2024-02-11", "09:00", "15:00", 6.0, "Schedule conflict cited", "Administrative Issue", "Email communication"),
        ("2024-02-14", "16:00", "20:00", 4.0, "Valentine's Day plans", "Holiday Priority", "Event documentation"),
        ("2024-02-18", "10:00", "18:00", 8.0, "President's Day trip", "Holiday Travel", "Travel itinerary"),
        ("2024-02-25", "14:00", "19:00", 5.0, "Transportation unavailable", "Logistical Issue", "Communication log"),
        
        // March 2024
        ("2024-03-03", "11:00", "16:00", 5.0, "Science fair prep", "Academic Priority", "Project materials"),
        ("2024-03-05", "09:00", "17:00", 8.0, "Calendar conflict", "Scheduling Issue", "Calendar evidence"),
        ("2024-03-12", "11:00", "16:00", 5.0, "Child preference expressed", "Behavioral Factor", "Statement recorded"),
        ("2024-03-17", "13:00", "20:00", 7.0, "St. Patrick's Day event", "Holiday Priority", "Event ticket"),
        ("2024-03-19", "15:00", "20:00", 5.0, "Activity commitment", "Priority Conflict", "Activity documentation"),
        ("2024-03-26", "09:00", "12:00", 3.0, "Weather concerns", "Environmental Factor", "Weather documentation"),
        ("2024-03-31", "16:00", "19:00", 3.0, "Easter preparation", "Holiday Prep", "Shopping receipt"),
        
        // April 2024
        ("2024-04-02", "13:00", "18:00", 5.0, "Vehicle unavailable", "Logistical Issue", "Maintenance documentation"),
        ("2024-04-07", "10:00", "17:00", 7.0, "Easter Sunday", "Holiday Conflict", "Family gathering"),
        ("2024-04-09", "10:00", "16:00", 6.0, "Medical appointment", "Health Priority", "Appointment verification"),
        ("2024-04-16", "14:00", "19:00", 5.0, "Family obligation", "Family Priority", "Family documentation"),
        ("2024-04-23", "09:00", "15:00", 6.0, "Educational event", "Academic Priority", "School notification"),
        ("2024-04-30", "16:00", "20:00", 4.0, "Emergency plans", "Unplanned Event", "Communication record"),
        
        // May 2024
        ("2024-05-05", "12:00", "18:00", 6.0, "Cinco de Mayo celebration", "Cultural Event", "Event flyer"),
        ("2024-05-07", "11:00", "17:00", 6.0, "Holiday preparation", "Holiday Conflict", "Activity receipts"),
        ("2024-05-12", "08:00", "20:00", 12.0, "Mother's Day denied", "Holiday Violation", "Text message denial"),
        ("2024-05-14", "09:00", "14:00", 5.0, "Social commitment", "Social Priority", "Event invitation"),
        ("2024-05-21", "15:00", "19:00", 4.0, "Child fatigue cited", "Health Concern", "Health log"),
        ("2024-05-27", "10:00", "18:00", 8.0, "Memorial Day weekend", "Holiday Priority", "Travel plans"),
        ("2024-05-28", "10:00", "16:00", 6.0, "Holiday plans", "Holiday Priority", "Family activity"),
        
        // June 2024
        ("2024-06-02", "14:00", "18:00", 4.0, "Graduation ceremony", "Academic Milestone", "Graduation notice"),
        ("2024-06-04", "13:00", "18:00", 5.0, "Lesson conflict", "Activity Conflict", "Lesson schedule"),
        ("2024-06-11", "09:00", "15:00", 6.0, "Special event", "Event Priority", "Event documentation"),
        ("2024-06-14", "08:00", "20:00", 12.0, "Holiday denied", "Holiday Conflict", "Communication record"),
        ("2024-06-16", "08:00", "20:00", 12.0, "Father's Day completely denied", "Holiday Violation", "Denial message"),
        ("2024-06-18", "16:00", "19:00", 3.0, "Social plans", "Social Priority", "Activity receipt"),
        ("2024-06-25", "11:00", "17:00", 6.0, "Program preparation", "Activity Preparation", "Program materials"),
        ("2024-06-29", "15:00", "20:00", 5.0, "Summer camp prep", "Activity Preparation", "Camp documentation"),
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
        ("2018-10-12", "court", "Initial Order Entered", "Initial custody arrangements established", 5),
        ("2023-12-05", "agreement", "Custody Stipulation Filed", "Stipulation establishing placement schedule", 4),
        ("2024-02-11", "violation", "First Incident Reported", "Beginning of documented placement issues", 5),
        ("2024-02-12", "mediation", "Mediator Directive Issued", "Formal directive regarding placement compliance", 4),
        ("2024-06-14", "violation", "Holiday Placement Denied", "Complete denial of holiday placement", 5),
        ("2024-06-26", "court", "Enforcement Motion Filed", "Legal action initiated for enforcement", 5),
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
        ("2024-02-11", "Placement Interference", "Systematic incident pattern begins", "Agreement Section 3.2", 4),
        ("2024-06-14", "Holiday Violation", "Holiday placement completely denied", "Agreement Section 2.1", 5),
        ("2024-06-26", "Compliance Pattern", "Ongoing non-compliance with agreements", "Multiple sections", 5),
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
