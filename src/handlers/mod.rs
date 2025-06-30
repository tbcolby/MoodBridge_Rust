use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html},
};

use serde_json::{json, Value};
use sqlx::Row;

// Re-export models
// use crate::models::*;
use crate::db::DbPool;

// Project management handlers module
pub mod project;

// Health check endpoint
pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "message": "Legal dashboard API is running"
    })))
}

// Project Management Dashboard HTML page
pub async fn project_dashboard() -> Html<String> {
    let html = std::fs::read_to_string("templates/project_dashboard.html")
        .unwrap_or_else(|_| {
            "<html><body><h1>Project Dashboard Template Not Found</h1><p>Please ensure templates/project_dashboard.html exists</p></body></html>".to_string()
        });
    Html(html)
}

// Dashboard HTML page
pub async fn dashboard() -> Html<String> {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MoodBridge Legal Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            color: #333;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            text-align: center;
            color: white;
            margin-bottom: 30px;
        }
        .header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .stat-card {
            background: white;
            padding: 25px;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
            text-align: center;
            transition: transform 0.3s ease;
        }
        .stat-card:hover {
            transform: translateY(-5px);
        }
        .stat-value {
            font-size: 2.5rem;
            font-weight: bold;
            color: #667eea;
            margin-bottom: 5px;
        }
        .stat-label {
            color: #666;
            font-size: 0.9rem;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        .charts-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .chart-card {
            background: white;
            padding: 25px;
            border-radius: 15px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
        }
        .chart-title {
            font-size: 1.2rem;
            font-weight: bold;
            margin-bottom: 15px;
            color: #333;
        }
        .recent-incidents {
            background: white;
            border-radius: 15px;
            padding: 25px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
        }
        .incident-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 15px 0;
            border-bottom: 1px solid #eee;
        }
        .incident-item:last-child {
            border-bottom: none;
        }
        .incident-date {
            font-weight: bold;
            color: #667eea;
        }
        .incident-reason {
            flex: 1;
            margin: 0 15px;
        }
        .incident-hours {
            background: #ff6b6b;
            color: white;
            padding: 5px 10px;
            border-radius: 20px;
            font-size: 0.8rem;
        }
        .loading {
            text-align: center;
            color: #666;
            padding: 40px;
        }
    </style>
</head>
<body>
        <div class="container">
            <div class="header">
                <h1>🦀⚖️ MoodBridge Legal Dashboard</h1>
                <p>Real-time analytics for family law case management</p>
                <div style="margin-top: 20px;">
                    <a href="/wizards" style="background: rgba(255,255,255,0.2); color: white; padding: 10px 20px; border-radius: 10px; text-decoration: none; margin-right: 15px;">🧙‍♂️ Wizard Engine</a>
                    <a href="/projects" style="background: rgba(255,255,255,0.2); color: white; padding: 10px 20px; border-radius: 10px; text-decoration: none;">📋 Project Manager</a>
                </div>
            </div>
        
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-value" id="total-incidents">...</div>
                <div class="stat-label">Total Incidents</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="total-hours">...</div>
                <div class="stat-label">Hours Lost</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="avg-duration">...</div>
                <div class="stat-label">Avg Duration</div>
            </div>
            <div class="stat-card">
                <div class="stat-value" id="this-month">...</div>
                <div class="stat-label">This Month</div>
            </div>
        </div>
        
        <div class="charts-grid">
            <div class="chart-card">
                <div class="chart-title">Monthly Incident Trend</div>
                <canvas id="monthlyChart" width="400" height="200"></canvas>
            </div>
            <div class="chart-card">
                <div class="chart-title">Incident Categories</div>
                <canvas id="categoryChart" width="400" height="200"></canvas>
            </div>
        </div>
        
        <div class="recent-incidents">
            <h2 style="margin-bottom: 20px;">Recent Incidents</h2>
            <div id="incidents-list" class="loading">Loading incidents...</div>
        </div>
    </div>
    
    <script>
        // Fetch dashboard data
        async function loadDashboard() {
            try {
                const response = await fetch('/api/dashboard-data');
                const data = await response.json();
                
                // Update stats
                document.getElementById('total-incidents').textContent = data.stats.total_incidents;
                document.getElementById('total-hours').textContent = data.stats.total_hours.toFixed(1);
                document.getElementById('avg-duration').textContent = data.stats.avg_duration.toFixed(1) + 'h';
                document.getElementById('this-month').textContent = data.stats.this_month;
                
                // Monthly trend chart
                const monthlyCtx = document.getElementById('monthlyChart').getContext('2d');
                new Chart(monthlyCtx, {
                    type: 'line',
                    data: {
                        labels: data.monthly_trend.map(item => item.month),
                        datasets: [{
                            label: 'Incidents',
                            data: data.monthly_trend.map(item => item.count),
                            borderColor: '#667eea',
                            backgroundColor: 'rgba(102, 126, 234, 0.1)',
                            tension: 0.4,
                            fill: true
                        }]
                    },
                    options: {
                        responsive: true,
                        plugins: { legend: { display: false } },
                        scales: {
                            y: { beginAtZero: true }
                        }
                    }
                });
                
                // Category pie chart
                const categoryCtx = document.getElementById('categoryChart').getContext('2d');
                new Chart(categoryCtx, {
                    type: 'doughnut',
                    data: {
                        labels: data.categories.map(item => item.category),
                        datasets: [{
                            data: data.categories.map(item => item.count),
                            backgroundColor: [
                                '#667eea', '#764ba2', '#f093fb', '#f5576c',
                                '#4facfe', '#00f2fe', '#43e97b', '#38f9d7'
                            ]
                        }]
                    },
                    options: {
                        responsive: true,
                        plugins: {
                            legend: { position: 'bottom' }
                        }
                    }
                });
                
                // Recent incidents
                const incidentsList = document.getElementById('incidents-list');
                incidentsList.innerHTML = data.recent_incidents.map(incident => `
                    <div class="incident-item">
                        <div class="incident-date">${incident.denied_date}</div>
                        <div class="incident-reason">${incident.denial_reason}</div>
                        <div class="incident-hours">${incident.duration_hours}h</div>
                    </div>
                `).join('');
                
            } catch (error) {
                console.error('Error loading dashboard:', error);
            }
        }
        
        // Load dashboard on page load
        document.addEventListener('DOMContentLoaded', loadDashboard);
    </script>
</body>
</html>
    "#;
    Html(html.to_string())
}

// Dashboard data API endpoint (simple version for axum 0.6)
pub async fn dashboard_data_simple(pool: DbPool) -> Result<Json<Value>, StatusCode> {
    dashboard_data_impl(&pool).await
}

// Dashboard data API endpoint
pub async fn dashboard_data(State(pool): State<DbPool>) -> Result<Json<Value>, StatusCode> {
    dashboard_data_impl(&pool).await
}

// Shared implementation
async fn dashboard_data_impl(pool: &DbPool) -> Result<Json<Value>, StatusCode> {
    // Get basic stats
    let stats_query = sqlx::query(
        "SELECT 
            COUNT(*) as total_incidents,
            COALESCE(SUM(duration_hours), 0) as total_hours,
            COALESCE(AVG(duration_hours), 0) as avg_duration,
            COUNT(CASE WHEN denied_date LIKE '2024-06%' THEN 1 END) as this_month
         FROM placement_denials"
    );
    
    let stats_row = stats_query.fetch_one(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
         ORDER BY month"
    );
    
    let monthly_rows = monthly_query.fetch_all(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let monthly_trend: Vec<Value> = monthly_rows.iter().map(|row: &sqlx::sqlite::SqliteRow| {
        json!({
            "month": row.get::<String, _>("month"),
            "count": row.get::<i64, _>("count")
        })
    }).collect();
    
    // Categories
    let category_query = sqlx::query(
        "SELECT 
            violation_category as category,
            COUNT(*) as count
         FROM placement_denials 
         GROUP BY violation_category
         ORDER BY count DESC"
    );
    
    let category_rows = category_query.fetch_all(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let categories: Vec<Value> = category_rows.iter().map(|row: &sqlx::sqlite::SqliteRow| {
        json!({
            "category": row.get::<String, _>("category"),
            "count": row.get::<i64, _>("count")
        })
    }).collect();
    
    // Recent incidents
    let recent_query = sqlx::query(
        "SELECT denied_date, denial_reason, duration_hours
         FROM placement_denials 
         ORDER BY denied_date DESC 
         LIMIT 10"
    );
    
    let recent_rows = recent_query.fetch_all(pool).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let recent_incidents: Vec<Value> = recent_rows.iter().map(|row: &sqlx::sqlite::SqliteRow| {
        json!({
            "denied_date": row.get::<String, _>("denied_date"),
            "denial_reason": row.get::<String, _>("denial_reason"),
            "duration_hours": row.get::<f64, _>("duration_hours")
        })
    }).collect();
    
    Ok(Json(json!({
        "stats": stats,
        "monthly_trend": monthly_trend,
        "categories": categories,
        "recent_incidents": recent_incidents
    })))
}
