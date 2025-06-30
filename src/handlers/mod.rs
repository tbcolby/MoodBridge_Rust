// Placeholder for API handlers - will be implemented in future iterations
// This will contain handlers for:
// - Dashboard metrics
// - Placement denials CRUD
// - Timeline events
// - Exhibits management
// - Analytics queries

use axum::response::Json;
use crate::models::*;

pub async fn get_dashboard_metrics() -> Json<&'static str> {
    Json("Dashboard metrics endpoint - coming soon!")
}

pub async fn get_placement_denials() -> Json<&'static str> {
    Json("Placement denials endpoint - coming soon!")
}
