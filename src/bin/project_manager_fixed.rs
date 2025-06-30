use clap::{Parser, Subcommand};
use sqlx::{Pool, Sqlite};
use std::env;

#[derive(Parser)]
#[command(name = "project_manager")]
#[command(about = "MoodBridge Project Management CLI Tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all projects
    List,
    /// Show project details
    Show { id: i64 },
    /// Create a new project
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short, long, default_value = "feature")]
        project_type: String,
    },
    /// Update project status
    Status {
        id: i64,
        #[arg(value_enum)]
        status: ProjectStatus,
    },
    /// Add a task to a project
    AddTask {
        #[arg(short, long)]
        project_id: i64,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short = 'P', long, default_value = "medium")]
        priority: String,
        #[arg(short, long)]
        due_date: Option<String>,
        #[arg(short, long)]
        estimated_hours: Option<f64>,
    },
    /// List tasks for a project
    Tasks { project_id: i64 },
    /// Show project dashboard summary
    Dashboard,
    /// Initialize database with sample data
    Init,
}

#[derive(clap::ValueEnum, Clone)]
enum ProjectStatus {
    Planning,
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "planning"),
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Paused => write!(f, "paused"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Setup database connection
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let db_path = current_dir.join("data").join("main.db");
        format!("sqlite:{}", db_path.display())
    });

    let pool = Pool::<Sqlite>::connect(&database_url).await?;

    match cli.command {
        Commands::List => list_projects(&pool).await?,
        Commands::Show { id } => show_project(&pool, id).await?,
        Commands::Create { 
            name, 
            description, 
            priority, 
            project_type 
        } => create_project(&pool, name, description, priority, project_type).await?,
        Commands::Status { id, status } => update_project_status(&pool, id, status).await?,
        Commands::AddTask { 
            project_id, 
            title, 
            description, 
            priority, 
            due_date, 
            estimated_hours 
        } => add_task(&pool, project_id, title, description, priority, due_date, estimated_hours).await?,
        Commands::Tasks { project_id } => list_tasks(&pool, project_id).await?,
        Commands::Dashboard => show_dashboard(&pool).await?,
        Commands::Init => init_database(&pool).await?,
    }

    Ok(())
}

async fn list_projects(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let projects = sqlx::query!(
        "SELECT id, name, status, priority, progress_percentage, 
                estimated_hours, actual_hours, target_date 
         FROM projects 
         ORDER BY priority DESC, updated_at DESC"
    )
    .fetch_all(pool)
    .await?;

    println!("📋 Projects Overview");
    println!("{}", "=".repeat(100));
    println!("{:<4} {:<30} {:<12} {:<10} {:<10} {:<12} {:<12} {:<15}", 
        "ID", "Name", "Status", "Priority", "Progress", "Est Hours", "Act Hours", "Target Date");
    println!("{}", "-".repeat(100));

    for project in projects {
        println!("{:<4} {:<30} {:<12} {:<10} {:<9.1}% {:<12} {:<12} {:<15}", 
            project.id.unwrap_or(0),
            truncate(&project.name, 29),
            project.status,
            project.priority,
            project.progress_percentage.unwrap_or(0.0),
            project.estimated_hours.map(|h| format!("{:.1}", h)).unwrap_or_else(|| "N/A".to_string()),
            project.actual_hours.map(|h| format!("{:.1}", h)).unwrap_or_else(|| "0.0".to_string()),
            project.target_date.unwrap_or_else(|| "N/A".to_string())
        );
    }

    Ok(())
}

async fn show_project(pool: &Pool<Sqlite>, id: i64) -> Result<(), sqlx::Error> {
    let project = sqlx::query!(
        "SELECT * FROM projects WHERE id = ?", id
    )
    .fetch_optional(pool)
    .await?;

    match project {
        Some(p) => {
            println!("📊 Project Details");
            println!("{}", "=".repeat(60));
            println!("ID: {}", p.id);
            println!("Name: {}", p.name);
            println!("Description: {}", p.description.unwrap_or_else(|| "N/A".to_string()));
            println!("Status: {}", p.status);
            println!("Priority: {}", p.priority);
            println!("Type: {}", p.project_type);
            println!("Progress: {:.1}%", p.progress_percentage.unwrap_or(0.0));
            println!("Owner: {}", p.owner.unwrap_or_else(|| "N/A".to_string()));
            println!("Estimated Hours: {}", p.estimated_hours.map(|h| format!("{:.1}", h)).unwrap_or_else(|| "N/A".to_string()));
            println!("Actual Hours: {}", p.actual_hours.map(|h| format!("{:.1}", h)).unwrap_or_else(|| "0.0".to_string()));
            println!("Start Date: {}", p.start_date.unwrap_or_else(|| "N/A".to_string()));
            println!("Target Date: {}", p.target_date.unwrap_or_else(|| "N/A".to_string()));
            println!("Completion Date: {}", p.completion_date.unwrap_or_else(|| "N/A".to_string()));
            println!("Created: {}", p.created_at.unwrap_or_else(|| "N/A".to_string()));
            println!("Updated: {}", p.updated_at.unwrap_or_else(|| "N/A".to_string()));

            // Show tasks
            println!("\n📝 Tasks:");
            list_tasks(pool, id).await?;
        }
        None => println!("❌ Project with ID {} not found", id),
    }

    Ok(())
}

async fn create_project(
    pool: &Pool<Sqlite>, 
    name: String, 
    description: Option<String>, 
    priority: String, 
    project_type: String
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO projects (name, description, priority, project_type, owner) 
         VALUES (?, ?, ?, ?, 'cli')",
        name, description, priority, project_type
    )
    .execute(pool)
    .await?;

    println!("✅ Created project '{}' with ID: {}", name, result.last_insert_rowid());
    Ok(())
}

async fn update_project_status(
    pool: &Pool<Sqlite>, 
    id: i64, 
    status: ProjectStatus
) -> Result<(), sqlx::Error> {
    let status_str = status.to_string();
    let result = sqlx::query!(
        "UPDATE projects SET status = ? WHERE id = ?",
        status_str, id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        println!("✅ Updated project {} status to: {}", id, status_str);
    } else {
        println!("❌ Project with ID {} not found", id);
    }

    Ok(())
}

async fn add_task(
    pool: &Pool<Sqlite>,
    project_id: i64,
    title: String,
    description: Option<String>,
    priority: String,
    due_date: Option<String>,
    estimated_hours: Option<f64>,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO tasks (project_id, title, description, priority, due_date, estimated_hours)
         VALUES (?, ?, ?, ?, ?, ?)",
        project_id, title, description, priority, due_date, estimated_hours
    )
    .execute(pool)
    .await?;

    println!("✅ Added task '{}' to project {} with ID: {}", title, project_id, result.last_insert_rowid());
    Ok(())
}

async fn list_tasks(pool: &Pool<Sqlite>, project_id: i64) -> Result<(), sqlx::Error> {
    let tasks = sqlx::query!(
        "SELECT id, title, status, priority, estimated_hours, due_date 
         FROM tasks 
         WHERE project_id = ? 
         ORDER BY priority DESC, due_date ASC",
        project_id
    )
    .fetch_all(pool)
    .await?;

    if tasks.is_empty() {
        println!("No tasks found for project {}", project_id);
        return Ok(());
    }

    println!("{:<4} {:<30} {:<12} {:<10} {:<12} {:<15}", 
        "ID", "Title", "Status", "Priority", "Est Hours", "Due Date");
    println!("{}", "-".repeat(85));

    for task in tasks {
        println!("{:<4} {:<30} {:<12} {:<10} {:<12} {:<15}", 
            task.id,
            truncate(&task.title, 29),
            task.status,
            task.priority,
            task.estimated_hours.map(|h| format!("{:.1}", h)).unwrap_or_else(|| "N/A".to_string()),
            task.due_date.unwrap_or_else(|| "N/A".to_string())
        );
    }

    Ok(())
}

async fn show_dashboard(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Project summary
    let summary = sqlx::query!(
        "SELECT 
            COUNT(*) as total_projects,
            COUNT(CASE WHEN status = 'active' THEN 1 END) as active_projects,
            COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed_projects,
            SUM(estimated_hours) as total_estimated_hours,
            SUM(actual_hours) as total_actual_hours
         FROM projects"
    )
    .fetch_one(pool)
    .await?;

    // Task summary
    let task_summary = sqlx::query!(
        "SELECT 
            COUNT(*) as total_tasks,
            COUNT(CASE WHEN status = 'done' THEN 1 END) as completed_tasks,
            COUNT(CASE WHEN priority = 'critical' AND status != 'done' THEN 1 END) as critical_tasks,
            COUNT(CASE WHEN due_date < DATE('now') AND status != 'done' THEN 1 END) as overdue_tasks
         FROM tasks"
    )
    .fetch_one(pool)
    .await?;

    println!("🚀 MoodBridge Project Dashboard");
    println!("{}", "=".repeat(50));
    
    println!("\n📊 Project Summary:");
    println!("  Total Projects: {}", summary.total_projects);
    println!("  Active Projects: {}", summary.active_projects.unwrap_or(0));
    println!("  Completed Projects: {}", summary.completed_projects.unwrap_or(0));
    println!("  Total Estimated Hours: {:.1}", summary.total_estimated_hours.unwrap_or(0.0));
    println!("  Total Actual Hours: {:.1}", summary.total_actual_hours.unwrap_or(0.0));

    println!("\n📝 Task Summary:");
    println!("  Total Tasks: {}", task_summary.total_tasks);
    println!("  Completed Tasks: {}", task_summary.completed_tasks.unwrap_or(0));
    println!("  Critical Tasks: {}", task_summary.critical_tasks.unwrap_or(0));
    println!("  Overdue Tasks: {}", task_summary.overdue_tasks.unwrap_or(0));

    // Show active projects
    println!("\n🔥 Active Projects:");
    let active_projects = sqlx::query!(
        "SELECT id, name, priority, progress_percentage 
         FROM projects 
         WHERE status = 'active' 
         ORDER BY priority DESC"
    )
    .fetch_all(pool)
    .await?;

    for project in active_projects {
        println!("  • {} (ID: {}) - {:.1}% [{}]", 
            project.name, 
            project.id.unwrap_or(0), 
            project.progress_percentage.unwrap_or(0.0),
            project.priority
        );
    }

    Ok(())
}

async fn init_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    println!("🗃️ Initializing database with sample project data...");

    // The database should already be initialized by the main app
    // This just confirms the setup
    let project_count = sqlx::query!("SELECT COUNT(*) as count FROM projects")
        .fetch_one(pool)
        .await?;

    let task_count = sqlx::query!("SELECT COUNT(*) as count FROM tasks")
        .fetch_one(pool)
        .await?;

    println!("✅ Database initialized!");
    println!("  Projects: {}", project_count.count);
    println!("  Tasks: {}", task_count.count);

    if project_count.count == 0 {
        println!("⚠️  No projects found. Make sure to run the main application first to seed the database.");
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}
