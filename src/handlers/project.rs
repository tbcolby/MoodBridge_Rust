use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;

use crate::models::{
    Project, Task, Milestone, WorkSession, ProjectSummary, 
    TasksByStatus, ProjectProgress, ProductivityMetrics
};

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub project_type: Option<String>,
    pub owner: Option<String>,
    pub estimated_hours: Option<f64>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub progress_percentage: Option<f64>,
    pub actual_hours: Option<f64>,
    pub completion_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub task_type: Option<String>,
    pub assignee: Option<String>,
    pub estimated_hours: Option<f64>,
    pub due_date: Option<String>,
    pub dependencies: Option<Vec<i64>>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub actual_hours: Option<f64>,
    pub completion_date: Option<String>,
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub project_type: Option<String>,
    pub assignee: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ProjectWithTasks {
    #[serde(flatten)]
    pub project: Project,
    pub tasks: Vec<Task>,
    pub milestones: Vec<Milestone>,
    pub task_counts: HashMap<String, i64>,
}

#[derive(Debug, Serialize)]
pub struct DashboardData {
    pub summary: ProjectSummary,
    pub active_projects: Vec<ProjectProgress>,
    pub urgent_tasks: Vec<Task>,
    pub upcoming_milestones: Vec<Milestone>,
    pub recent_activity: Vec<WorkSession>,
    pub productivity_trend: Vec<ProductivityMetrics>,
}

// Project CRUD handlers
pub async fn get_projects(
    State(pool): State<Pool<Sqlite>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    let mut query = "SELECT * FROM projects WHERE 1=1".to_string();
    let mut conditions = Vec::new();

    if let Some(status) = &params.status {
        query.push_str(" AND status = ?");
        conditions.push(status.as_str());
    }
    if let Some(priority) = &params.priority {
        query.push_str(" AND priority = ?");
        conditions.push(priority.as_str());
    }
    if let Some(project_type) = &params.project_type {
        query.push_str(" AND project_type = ?");
        conditions.push(project_type.as_str());
    }

    query.push_str(" ORDER BY priority DESC, updated_at DESC");
    
    if let Some(limit) = params.limit {
        query.push_str(&format!(" LIMIT {}", limit));
        if let Some(offset) = params.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
    }

    let mut sql_query = sqlx::query_as::<_, Project>(&query);
    for condition in conditions {
        sql_query = sql_query.bind(condition);
    }

    match sql_query.fetch_all(&pool).await {
        Ok(projects) => Ok(Json(projects)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_project(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<ProjectWithTasks>, StatusCode> {
    // Get project
    let project = match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(project) => project,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    // Get tasks
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE project_id = ? ORDER BY priority DESC, due_date ASC")
        .bind(id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // Get milestones
    let milestones = sqlx::query_as::<_, Milestone>("SELECT * FROM milestones WHERE project_id = ? ORDER BY target_date ASC")
        .bind(id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    // Get task counts by status
    let task_status_counts = sqlx::query_as::<_, TasksByStatus>(
        "SELECT status, COUNT(*) as task_count, SUM(estimated_hours) as estimated_hours 
         FROM tasks WHERE project_id = ? GROUP BY status"
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let mut task_counts = HashMap::new();
    for count in task_status_counts {
        task_counts.insert(count.status, count.task_count);
    }

    Ok(Json(ProjectWithTasks {
        project,
        tasks,
        milestones,
        task_counts,
    }))
}

pub async fn create_project(
    State(pool): State<Pool<Sqlite>>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    let tags_json = request.tags
        .map(|tags| serde_json::to_string(&tags).unwrap_or_default());

    let result = sqlx::query(
        "INSERT INTO projects (name, description, status, priority, project_type, owner, estimated_hours, start_date, target_date, tags)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&request.name)
    .bind(&request.description)
    .bind(request.status.unwrap_or_else(|| "planning".to_string()))
    .bind(request.priority.unwrap_or_else(|| "medium".to_string()))
    .bind(request.project_type.unwrap_or_else(|| "feature".to_string()))
    .bind(&request.owner)
    .bind(request.estimated_hours)
    .bind(&request.start_date)
    .bind(&request.target_date)
    .bind(&tags_json)
    .execute(&pool)
    .await;

    match result {
        Ok(result) => {
            let id = result.last_insert_rowid();
            match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(project) => Ok(Json(project)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_project(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateProjectRequest>,
) -> Result<Json<Project>, StatusCode> {
    let mut updates = Vec::new();
    let mut values = Vec::new();

    if let Some(name) = &request.name {
        updates.push("name = ?");
        values.push(name.as_str());
    }
    if let Some(description) = &request.description {
        updates.push("description = ?");
        values.push(description.as_str());
    }
    if let Some(status) = &request.status {
        updates.push("status = ?");
        values.push(status.as_str());
    }
    if let Some(priority) = &request.priority {
        updates.push("priority = ?");
        values.push(priority.as_str());
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let query = format!("UPDATE projects SET {} WHERE id = ?", updates.join(", "));
    let mut sql_query = sqlx::query(&query);
    
    for value in values {
        sql_query = sql_query.bind(value);
    }
    sql_query = sql_query.bind(id);

    match sql_query.execute(&pool).await {
        Ok(_) => {
            match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(project) => Ok(Json(project)),
                Err(_) => Err(StatusCode::NOT_FOUND),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Task CRUD handlers
pub async fn get_tasks(
    State(pool): State<Pool<Sqlite>>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<Task>>, StatusCode> {
    let mut query = "SELECT * FROM tasks WHERE 1=1".to_string();
    let mut conditions = Vec::new();

    if let Some(status) = &params.status {
        query.push_str(" AND status = ?");
        conditions.push(status.as_str());
    }
    if let Some(priority) = &params.priority {
        query.push_str(" AND priority = ?");
        conditions.push(priority.as_str());
    }
    if let Some(assignee) = &params.assignee {
        query.push_str(" AND assignee = ?");
        conditions.push(assignee.as_str());
    }

    query.push_str(" ORDER BY priority DESC, due_date ASC");
    
    if let Some(limit) = params.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    let mut sql_query = sqlx::query_as::<_, Task>(&query);
    for condition in conditions {
        sql_query = sql_query.bind(condition);
    }

    match sql_query.fetch_all(&pool).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_task(
    State(pool): State<Pool<Sqlite>>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, StatusCode> {
    let dependencies_json = request.dependencies
        .map(|deps| serde_json::to_string(&deps).unwrap_or_default());
    let labels_json = request.labels
        .map(|labels| serde_json::to_string(&labels).unwrap_or_default());

    let result = sqlx::query(
        "INSERT INTO tasks (project_id, title, description, status, priority, task_type, assignee, estimated_hours, due_date, dependencies, labels)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(request.project_id)
    .bind(&request.title)
    .bind(&request.description)
    .bind(request.status.unwrap_or_else(|| "todo".to_string()))
    .bind(request.priority.unwrap_or_else(|| "medium".to_string()))
    .bind(request.task_type.unwrap_or_else(|| "implementation".to_string()))
    .bind(&request.assignee)
    .bind(request.estimated_hours)
    .bind(&request.due_date)
    .bind(&dependencies_json)
    .bind(&labels_json)
    .execute(&pool)
    .await;

    match result {
        Ok(result) => {
            let id = result.last_insert_rowid();
            match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(task) => Ok(Json(task)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_task(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<i64>,
    Json(request): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, StatusCode> {
    let mut updates = Vec::new();
    let mut values = Vec::new();

    if let Some(title) = &request.title {
        updates.push("title = ?");
        values.push(title.as_str());
    }
    if let Some(status) = &request.status {
        updates.push("status = ?");
        values.push(status.as_str());
        
        // Auto-set completion date when status changes to 'done'
        if status == "done" {
            updates.push("completion_date = CURRENT_TIMESTAMP");
        }
    }
    if let Some(priority) = &request.priority {
        updates.push("priority = ?");
        values.push(priority.as_str());
    }
    if let Some(blocked_reason) = &request.blocked_reason {
        updates.push("blocked_reason = ?");
        values.push(blocked_reason.as_str());
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let query = format!("UPDATE tasks SET {} WHERE id = ?", updates.join(", "));
    let mut sql_query = sqlx::query(&query);
    
    for value in values {
        sql_query = sql_query.bind(value);
    }
    sql_query = sql_query.bind(id);

    match sql_query.execute(&pool).await {
        Ok(_) => {
            match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(task) => Ok(Json(task)),
                Err(_) => Err(StatusCode::NOT_FOUND),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Analytics and Dashboard handlers
pub async fn get_project_dashboard(
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<DashboardData>, StatusCode> {
    // Get project summary
    let summary = sqlx::query_as::<_, ProjectSummary>(
        "SELECT 
            COUNT(*) as total_projects,
            COUNT(CASE WHEN status = 'active' THEN 1 END) as active_projects,
            COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed_projects,
            (SELECT COUNT(*) FROM tasks WHERE due_date < DATE('now') AND status != 'done') as overdue_tasks,
            (SELECT COUNT(*) FROM tasks WHERE priority = 'critical' AND status != 'done') as critical_tasks,
            SUM(estimated_hours) as total_estimated_hours,
            SUM(actual_hours) as total_actual_hours
         FROM projects"
    )
    .fetch_one(&pool)
    .await
    .unwrap_or_else(|_| ProjectSummary {
        total_projects: 0,
        active_projects: 0,
        completed_projects: 0,
        overdue_tasks: 0,
        critical_tasks: 0,
        total_estimated_hours: None,
        total_actual_hours: None,
    });

    // Get active projects with progress
    let active_projects = sqlx::query_as::<_, ProjectProgress>(
        "SELECT 
            p.id as project_id,
            p.name as project_name,
            COUNT(t.id) as total_tasks,
            COUNT(CASE WHEN t.status = 'done' THEN 1 END) as completed_tasks,
            ROUND(CAST(COUNT(CASE WHEN t.status = 'done' THEN 1 END) AS FLOAT) * 100.0 / COUNT(t.id), 2) as progress_percentage,
            SUM(t.estimated_hours) as estimated_hours,
            SUM(t.actual_hours) as actual_hours,
            JULIANDAY(p.target_date) - JULIANDAY('now') as days_remaining
         FROM projects p
         LEFT JOIN tasks t ON p.id = t.project_id
         WHERE p.status = 'active'
         GROUP BY p.id, p.name, p.target_date
         ORDER BY p.priority DESC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Get urgent tasks (critical priority or due soon)
    let urgent_tasks = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks 
         WHERE status != 'done' 
         AND (priority = 'critical' OR due_date <= DATE('now', '+2 days'))
         ORDER BY priority DESC, due_date ASC
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Get upcoming milestones
    let upcoming_milestones = sqlx::query_as::<_, Milestone>(
        "SELECT * FROM milestones 
         WHERE status != 'completed'
         AND target_date >= DATE('now')
         ORDER BY target_date ASC
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Get recent work sessions
    let recent_activity = sqlx::query_as::<_, WorkSession>(
        "SELECT * FROM work_sessions 
         ORDER BY start_time DESC 
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Productivity trend (placeholder for now)
    let productivity_trend = Vec::new();

    Ok(Json(DashboardData {
        summary,
        active_projects,
        urgent_tasks,
        upcoming_milestones,
        recent_activity,
        productivity_trend,
    }))
}

pub async fn get_task_analytics(
    State(pool): State<Pool<Sqlite>>,
) -> Result<Json<Vec<TasksByStatus>>, StatusCode> {
    let task_analytics = sqlx::query_as::<_, TasksByStatus>(
        "SELECT 
            status,
            COUNT(*) as task_count,
            SUM(estimated_hours) as estimated_hours
         FROM tasks 
         GROUP BY status
         ORDER BY task_count DESC"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Ok(Json(task_analytics))
}

// Work session tracking
pub async fn start_work_session(
    State(pool): State<Pool<Sqlite>>,
    Path(task_id): Path<i64>,
) -> Result<Json<WorkSession>, StatusCode> {
    let result = sqlx::query(
        "INSERT INTO work_sessions (task_id, start_time, session_type)
         VALUES (?, CURRENT_TIMESTAMP, 'focused')"
    )
    .bind(task_id)
    .execute(&pool)
    .await;

    match result {
        Ok(result) => {
            let id = result.last_insert_rowid();
            match sqlx::query_as::<_, WorkSession>("SELECT * FROM work_sessions WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
            {
                Ok(session) => Ok(Json(session)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn end_work_session(
    State(pool): State<Pool<Sqlite>>,
    Path(session_id): Path<i64>,
    Json(notes): Json<Option<String>>,
) -> Result<Json<WorkSession>, StatusCode> {
    let result = sqlx::query(
        "UPDATE work_sessions 
         SET end_time = CURRENT_TIMESTAMP,
             duration_minutes = ROUND((JULIANDAY(CURRENT_TIMESTAMP) - JULIANDAY(start_time)) * 1440),
             notes = ?
         WHERE id = ?"
    )
    .bind(notes)
    .bind(session_id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            match sqlx::query_as::<_, WorkSession>("SELECT * FROM work_sessions WHERE id = ?")
                .bind(session_id)
                .fetch_one(&pool)
                .await
            {
                Ok(session) => Ok(Json(session)),
                Err(_) => Err(StatusCode::NOT_FOUND),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
