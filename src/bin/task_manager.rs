//! # MoodBridge Task Manager
//! 
//! A command-line tool for managing and tracking project tasks.
//! 
//! Usage:
//! - `cargo run --bin task_manager status` - Show overall project status
//! - `cargo run --bin task_manager next` - Show next tasks to work on
//! - `cargo run --bin task_manager phase <phase_num>` - Show tasks for specific phase
//! - `cargo run --bin task_manager complete <task_id>` - Mark task as complete
//! - `cargo run --bin task_manager urgent` - Show only urgent tasks

use std::collections::HashMap;
use std::env;
use std::fs;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use clap::{Arg, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: String,
    title: String,
    phase: String,
    category: String,
    priority: TaskPriority,
    status: TaskStatus,
    description: String,
    dependencies: Vec<String>,
    estimated_hours: Option<u32>,
    assigned_to: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Blocked { reason: String },
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectStatus {
    total_tasks: u32,
    completed_tasks: u32,
    in_progress_tasks: u32,
    pending_tasks: u32,
    blocked_tasks: u32,
    current_phase: String,
    progress_percentage: f64,
    estimated_completion: String,
    next_milestone: String,
}

impl Task {
    fn is_urgent(&self) -> bool {
        matches!(self.priority, TaskPriority::Critical) ||
        (matches!(self.priority, TaskPriority::High) && 
         matches!(self.status, TaskStatus::Pending))
    }

    fn can_start(&self, completed_tasks: &HashMap<String, Task>) -> bool {
        if !matches!(self.status, TaskStatus::Pending) {
            return false;
        }
        
        self.dependencies.iter().all(|dep_id| {
            completed_tasks.get(dep_id)
                .map(|task| matches!(task.status, TaskStatus::Completed))
                .unwrap_or(false)
        })
    }
}

fn load_tasks() -> Vec<Task> {
    // In a real implementation, this would load from the task file
    // For now, we'll create the task list based on our specification
    
    let mut tasks = Vec::new();
    let now = Utc::now();
    
    // Phase 1 tasks
    tasks.push(Task {
        id: "SEC-001".to_string(),
        title: "Implement OAuth2 client with authorization code flow".to_string(),
        phase: "Phase 1".to_string(),
        category: "Security Infrastructure".to_string(),
        priority: TaskPriority::Critical,
        status: TaskStatus::Pending,
        description: "Create OAuth2 client supporting authorization code flow for enterprise platforms".to_string(),
        dependencies: vec!["ARCH-001".to_string()],
        estimated_hours: Some(16),
        assigned_to: None,
        created_at: now,
        updated_at: now,
        due_date: None,
        tags: vec!["oauth2".to_string(), "security".to_string(), "authentication".to_string()],
    });
    
    tasks.push(Task {
        id: "SEC-002".to_string(),
        title: "Build token management and refresh system".to_string(),
        phase: "Phase 1".to_string(),
        category: "Security Infrastructure".to_string(),
        priority: TaskPriority::High,
        status: TaskStatus::Pending,
        description: "Implement secure token storage, refresh logic, and expiration handling".to_string(),
        dependencies: vec!["SEC-001".to_string()],
        estimated_hours: Some(12),
        assigned_to: None,
        created_at: now,
        updated_at: now,
        due_date: None,
        tags: vec!["tokens".to_string(), "security".to_string()],
    });
    
    tasks.push(Task {
        id: "ARCH-002".to_string(),
        title: "Complete PlatformIntegration trait implementation".to_string(),
        phase: "Phase 1".to_string(),
        category: "Integration Framework Design".to_string(),
        priority: TaskPriority::High,
        status: TaskStatus::Pending,
        description: "Finish implementation of core PlatformIntegration trait with all required methods".to_string(),
        dependencies: vec!["ARCH-001".to_string()],
        estimated_hours: Some(8),
        assigned_to: None,
        created_at: now,
        updated_at: now,
        due_date: None,
        tags: vec!["architecture".to_string(), "traits".to_string()],
    });
    
    tasks.push(Task {
        id: "OBS-001".to_string(),
        title: "Configure structured logging with tracing crate".to_string(),
        phase: "Phase 1".to_string(),
        category: "Observability Setup".to_string(),
        priority: TaskPriority::Medium,
        status: TaskStatus::Pending,
        description: "Set up structured logging using the tracing crate with proper log levels and formatting".to_string(),
        dependencies: vec![],
        estimated_hours: Some(6),
        assigned_to: None,
        created_at: now,
        updated_at: now,
        due_date: None,
        tags: vec!["logging".to_string(), "observability".to_string()],
    });
    
    // Add a completed task for demonstration
    tasks.push(Task {
        id: "ARCH-001".to_string(),
        title: "Designed core integration framework and traits".to_string(),
        phase: "Phase 1".to_string(),
        category: "Integration Framework Design".to_string(),
        priority: TaskPriority::Critical,
        status: TaskStatus::Completed,
        description: "Created the foundational trait definitions and architecture for platform integrations".to_string(),
        dependencies: vec![],
        estimated_hours: Some(20),
        assigned_to: None,
        created_at: now,
        updated_at: now,
        due_date: None,
        tags: vec!["architecture".to_string(), "foundation".to_string()],
    });
    
    tasks
}

fn calculate_project_status(tasks: &[Task]) -> ProjectStatus {
    let total_tasks = tasks.len() as u32;
    let completed_tasks = tasks.iter().filter(|t| matches!(t.status, TaskStatus::Completed)).count() as u32;
    let in_progress_tasks = tasks.iter().filter(|t| matches!(t.status, TaskStatus::InProgress)).count() as u32;
    let pending_tasks = tasks.iter().filter(|t| matches!(t.status, TaskStatus::Pending)).count() as u32;
    let blocked_tasks = tasks.iter().filter(|t| matches!(t.status, TaskStatus::Blocked { .. })).count() as u32;
    
    let progress_percentage = if total_tasks > 0 {
        (completed_tasks as f64 / total_tasks as f64) * 100.0
    } else {
        0.0
    };
    
    ProjectStatus {
        total_tasks: 157, // Total from specification
        completed_tasks,
        in_progress_tasks,
        pending_tasks,
        blocked_tasks,
        current_phase: "Phase 1 - Foundation & Architecture".to_string(),
        progress_percentage,
        estimated_completion: "October 2025".to_string(),
        next_milestone: "OAuth2 Authentication Framework".to_string(),
    }
}

fn show_status(tasks: &[Task]) {
    let status = calculate_project_status(tasks);
    
    println!("🚀 MoodBridge Rust - Project Status");
    println!("═══════════════════════════════════════");
    println!("📊 Overall Progress: {:.1}%", status.progress_percentage);
    println!("📋 Total Tasks: {}", status.total_tasks);
    println!("✅ Completed: {}", status.completed_tasks);
    println!("🚧 In Progress: {}", status.in_progress_tasks);
    println!("⏳ Pending: {}", status.pending_tasks);
    println!("🚫 Blocked: {}", status.blocked_tasks);
    println!("🎯 Current Phase: {}", status.current_phase);
    println!("🏁 Next Milestone: {}", status.next_milestone);
    println!("📅 Est. Completion: {}", status.estimated_completion);
    
    if status.total_tasks > 100 {
        println!("");
        println!("⚠️  WARNING: Large project with {} tasks detected!", status.total_tasks);
        println!("   Consider breaking down into smaller milestones.");
        println!("   Use 'cargo run --bin task_manager next' to see immediate actions.");
    }
}

fn show_urgent_tasks(tasks: &[Task]) {
    let urgent_tasks: Vec<_> = tasks.iter()
        .filter(|task| task.is_urgent())
        .collect();
    
    println!("🚨 URGENT TASKS ({} items)", urgent_tasks.len());
    println!("═══════════════════════════════════════");
    
    if urgent_tasks.is_empty() {
        println!("✅ No urgent tasks at the moment!");
        return;
    }
    
    for task in urgent_tasks {
        let priority_emoji = match task.priority {
            TaskPriority::Critical => "🔴",
            TaskPriority::High => "🟡",
            TaskPriority::Medium => "🟢",
            TaskPriority::Low => "⚪",
        };
        
        println!("{} [{}] {}", priority_emoji, task.id, task.title);
        println!("   Phase: {} | Category: {}", task.phase, task.category);
        if let Some(hours) = task.estimated_hours {
            println!("   Estimated: {} hours", hours);
        }
        println!("");
    }
}

fn show_next_tasks(tasks: &[Task]) {
    let completed_tasks: HashMap<String, Task> = tasks.iter()
        .filter(|t| matches!(t.status, TaskStatus::Completed))
        .map(|t| (t.id.clone(), t.clone()))
        .collect();
    
    let ready_tasks: Vec<_> = tasks.iter()
        .filter(|task| task.can_start(&completed_tasks))
        .take(10) // Show top 10 ready tasks
        .collect();
    
    println!("⏭️  NEXT AVAILABLE TASKS ({} ready)", ready_tasks.len());
    println!("═══════════════════════════════════════");
    
    if ready_tasks.is_empty() {
        println!("🚫 No tasks are currently available to start.");
        println!("   Check for blocked dependencies or completed tasks.");
        return;
    }
    
    for (i, task) in ready_tasks.iter().enumerate() {
        let priority_emoji = match task.priority {
            TaskPriority::Critical => "🔴",
            TaskPriority::High => "🟡",
            TaskPriority::Medium => "🟢",
            TaskPriority::Low => "⚪",
        };
        
        println!("{}. {} [{}] {}", i + 1, priority_emoji, task.id, task.title);
        println!("   Phase: {} | Category: {}", task.phase, task.category);
        
        if !task.dependencies.is_empty() {
            println!("   Dependencies: {}", task.dependencies.join(", "));
        }
        
        if let Some(hours) = task.estimated_hours {
            println!("   Estimated: {} hours", hours);
        }
        println!("");
    }
}

fn show_phase_tasks(tasks: &[Task], phase_num: &str) {
    let phase_filter = format!("Phase {}", phase_num);
    let phase_tasks: Vec<_> = tasks.iter()
        .filter(|task| task.phase.contains(&phase_filter))
        .collect();
    
    println!("📋 {} TASKS ({} items)", phase_filter.to_uppercase(), phase_tasks.len());
    println!("═══════════════════════════════════════");
    
    if phase_tasks.is_empty() {
        println!("No tasks found for {}", phase_filter);
        return;
    }
    
    let mut by_category: HashMap<String, Vec<&Task>> = HashMap::new();
    for task in phase_tasks {
        by_category.entry(task.category.clone()).or_default().push(task);
    }
    
    for (category, tasks) in by_category {
        println!("\n📂 {}", category);
        println!("───────────────────────────────────────");
        
        for task in tasks {
            let status_emoji = match task.status {
                TaskStatus::Completed => "✅",
                TaskStatus::InProgress => "🚧",
                TaskStatus::Pending => "⏳",
                TaskStatus::Blocked { .. } => "🚫",
                TaskStatus::Cancelled => "❌",
            };
            
            println!("{} [{}] {}", status_emoji, task.id, task.title);
            
            if let Some(hours) = task.estimated_hours {
                println!("   Est: {}h", hours);
            }
        }
    }
}

fn main() {
    let matches = Command::new("MoodBridge Task Manager")
        .version("1.0")
        .about("Manages MoodBridge Rust project tasks")
        .subcommand(
            Command::new("status")
                .about("Show overall project status")
        )
        .subcommand(
            Command::new("next")
                .about("Show next available tasks")
        )
        .subcommand(
            Command::new("urgent")
                .about("Show urgent tasks only")
        )
        .subcommand(
            Command::new("phase")
                .about("Show tasks for specific phase")
                .arg(Arg::new("phase_num")
                    .help("Phase number (1-6)")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            Command::new("complete")
                .about("Mark task as complete")
                .arg(Arg::new("task_id")
                    .help("Task ID to mark complete")
                    .required(true)
                    .index(1))
        )
        .get_matches();
    
    let tasks = load_tasks();
    
    match matches.subcommand() {
        Some(("status", _)) => show_status(&tasks),
        Some(("next", _)) => show_next_tasks(&tasks),
        Some(("urgent", _)) => show_urgent_tasks(&tasks),
        Some(("phase", sub_matches)) => {
            let phase_num = sub_matches.get_one::<String>("phase_num").unwrap();
            show_phase_tasks(&tasks, phase_num);
        },
        Some(("complete", sub_matches)) => {
            let task_id = sub_matches.get_one::<String>("task_id").unwrap();
            println!("Marking task {} as complete...", task_id);
            println!("(This would update the task file in a full implementation)");
        },
        _ => {
            println!("🤖 MoodBridge Task Manager");
            println!("═══════════════════════════════════════");
            println!("Available commands:");
            println!("  status  - Show project overview");
            println!("  next    - Show next tasks to work on");
            println!("  urgent  - Show urgent tasks only");
            println!("  phase N - Show tasks for phase N");
            println!("");
            println!("Usage: cargo run --bin task_manager <command>");
            println!("");
            show_status(&tasks);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_urgency() {
        let task = Task {
            id: "TEST-001".to_string(),
            title: "Test task".to_string(),
            phase: "Test".to_string(),
            category: "Testing".to_string(),
            priority: TaskPriority::Critical,
            status: TaskStatus::Pending,
            description: "Test".to_string(),
            dependencies: vec![],
            estimated_hours: None,
            assigned_to: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            tags: vec![],
        };
        
        assert!(task.is_urgent());
    }

    #[test]
    fn test_task_can_start() {
        let task = Task {
            id: "TEST-002".to_string(),
            title: "Test task with deps".to_string(),
            phase: "Test".to_string(),
            category: "Testing".to_string(),
            priority: TaskPriority::Medium,
            status: TaskStatus::Pending,
            description: "Test".to_string(),
            dependencies: vec!["TEST-001".to_string()],
            estimated_hours: None,
            assigned_to: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            due_date: None,
            tags: vec![],
        };
        
        let completed_tasks = HashMap::new();
        assert!(!task.can_start(&completed_tasks));
    }

    #[test]
    fn test_project_status_calculation() {
        let tasks = load_tasks();
        let status = calculate_project_status(&tasks);
        
        assert!(status.total_tasks > 0);
        assert!(status.progress_percentage >= 0.0);
        assert!(status.progress_percentage <= 100.0);
    }
}
