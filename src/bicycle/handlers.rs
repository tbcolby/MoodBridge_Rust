//! # Bicycle Design System API Handlers
//!
//! This module provides REST API endpoints for the bicycle design and testing system,
//! including design management, simulation execution, and results visualization.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    BicycleSystem, BicycleSystemConfig,
    design::{BicycleDesign, BicycleType, ValidationStatus},
    simulation::SimulationResult,
    physics::{PhysicsEngine, MotionSimulation},
    materials::{Material, MaterialComparison, MaterialSelectionCriteria, MaterialApplication},
};

/// API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Operation successful".to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Design creation request
#[derive(Debug, Deserialize)]
pub struct CreateDesignRequest {
    pub name: String,
    pub bicycle_type: BicycleType,
    pub target_weight: Option<f64>,
    pub target_price: Option<f64>,
}

/// Design update request
#[derive(Debug, Deserialize)]
pub struct UpdateDesignRequest {
    pub name: Option<String>,
    pub target_weight: Option<f64>,
    pub target_price: Option<f64>,
    pub validation_status: Option<ValidationStatus>,
}

/// Simulation execution request
#[derive(Debug, Deserialize)]
pub struct RunSimulationRequest {
    pub design_id: Uuid,
    pub config: Option<BicycleSystemConfig>,
    pub scenarios: Vec<SimulationScenario>,
}

#[derive(Debug, Deserialize)]
pub struct SimulationScenario {
    pub name: String,
    pub conditions: SimulationConditions,
}

#[derive(Debug, Deserialize)]
pub struct SimulationConditions {
    pub rider_weight: f64,      // kg
    pub rider_power: f64,       // watts
    pub wind_speed: f64,        // m/s
    pub gradient: f64,          // percentage
    pub surface_type: String,   // "asphalt", "gravel", etc.
    pub temperature: f64,       // celsius
}

/// Material selection request
#[derive(Debug, Deserialize)]
pub struct MaterialSelectionRequest {
    pub application: MaterialApplication,
    pub criteria: MaterialSelectionCriteria,
    pub max_cost: Option<f64>,
    pub eco_friendly_only: Option<bool>,
}

/// Design comparison request
#[derive(Debug, Deserialize)]
pub struct CompareDesignsRequest {
    pub design_ids: Vec<Uuid>,
    pub comparison_criteria: Vec<String>,
}

/// Performance analysis result
#[derive(Debug, Serialize)]
pub struct PerformanceAnalysis {
    pub design_id: Uuid,
    pub overall_score: f64,
    pub performance_metrics: HashMap<String, f64>,
    pub recommendations: Vec<String>,
    pub optimal_use_cases: Vec<String>,
}

/// System state for handlers
pub struct BicycleSystemState {
    pub bicycle_system: BicycleSystem,
    pub physics_engine: PhysicsEngine,
}

impl BicycleSystemState {
    pub fn new() -> Self {
        let config = BicycleSystemConfig::default();
        Self {
            bicycle_system: BicycleSystem::new(config),
            physics_engine: PhysicsEngine::new(),
        }
    }
}

/// Create the bicycle API router
pub fn create_bicycle_router() -> Router<SqlitePool> {
    Router::new()
        // Design management endpoints
        .route("/api/bicycle/designs", get(get_designs).post(create_design))
        .route("/api/bicycle/designs/:id", get(get_design).put(update_design).delete(delete_design))
        .route("/api/bicycle/designs/:id/validate", post(validate_design))
        .route("/api/bicycle/designs/:id/clone", post(clone_design))
        
        // Simulation endpoints
        .route("/api/bicycle/simulations", post(run_simulation))
        .route("/api/bicycle/simulations/:id", get(get_simulation_result))
        .route("/api/bicycle/simulations/:id/analysis", get(get_performance_analysis))
        
        // Material selection endpoints
        .route("/api/bicycle/materials", get(get_materials))
        .route("/api/bicycle/materials/select", post(select_materials))
        .route("/api/bicycle/materials/compare", post(compare_materials))
        
        // Comparison and optimization endpoints
        .route("/api/bicycle/designs/compare", post(compare_designs))
        .route("/api/bicycle/designs/:id/optimize", post(optimize_design))
        
        // Analytics and reporting endpoints
        .route("/api/bicycle/analytics/summary", get(get_analytics_summary))
        .route("/api/bicycle/reports/performance", get(generate_performance_report))
        
        // Configuration endpoints
        .route("/api/bicycle/config", get(get_system_config).put(update_system_config))
        
        // Health check
        .route("/api/bicycle/health", get(health_check))
}

/// Get all designs
pub async fn get_designs(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<Vec<BicycleDesign>>>, StatusCode> {
    // In a real implementation, this would query the database
    // For now, return empty list
    let designs = Vec::new();
    Ok(Json(ApiResponse::success(designs)))
}

/// Create a new design
pub async fn create_design(
    State(pool): State<SqlitePool>,
    Json(request): Json<CreateDesignRequest>,
) -> Result<Json<ApiResponse<BicycleDesign>>, StatusCode> {
    let mut design = BicycleDesign::new(&request.name);
    design.intended_use = request.bicycle_type;
    
    if let Some(weight) = request.target_weight {
        design.target_weight = weight;
    }
    
    if let Some(price) = request.target_price {
        design.target_price = price;
    }

    // In a real implementation, save to database
    tracing::info!("Created new bicycle design: {} ({})", design.name, design.id);
    
    Ok(Json(ApiResponse::success(design)))
}

/// Get a specific design
pub async fn get_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<BicycleDesign>>, StatusCode> {
    // In a real implementation, query database for design
    let design = BicycleDesign::new("Sample Design");
    Ok(Json(ApiResponse::success(design)))
}

/// Update a design
pub async fn update_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateDesignRequest>,
) -> Result<Json<ApiResponse<BicycleDesign>>, StatusCode> {
    // In a real implementation, update database
    let mut design = BicycleDesign::new("Updated Design");
    design.id = id;
    design.updated_at = chrono::Utc::now();
    
    if let Some(name) = request.name {
        design.name = name;
    }
    
    if let Some(weight) = request.target_weight {
        design.target_weight = weight;
    }
    
    if let Some(price) = request.target_price {
        design.target_price = price;
    }
    
    if let Some(status) = request.validation_status {
        design.validation_status = status;
    }

    Ok(Json(ApiResponse::success(design)))
}

/// Delete a design
pub async fn delete_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // In a real implementation, delete from database
    tracing::info!("Deleted bicycle design: {}", id);
    Ok(Json(ApiResponse::success(format!("Design {} deleted successfully", id))))
}

/// Validate a design
pub async fn validate_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    // In a real implementation, load design and validate
    let design = BicycleDesign::new("Sample Design");
    let validation_issues = design.validate_design();
    
    if validation_issues.is_empty() {
        tracing::info!("Design {} validation passed", id);
    } else {
        tracing::warn!("Design {} validation failed with {} issues", id, validation_issues.len());
    }
    
    Ok(Json(ApiResponse::success(validation_issues)))
}

/// Clone a design
pub async fn clone_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<BicycleDesign>>, StatusCode> {
    // In a real implementation, load original design and clone
    let mut cloned_design = BicycleDesign::new("Cloned Design");
    cloned_design.id = Uuid::new_v4();
    cloned_design.version = 1;
    cloned_design.created_at = chrono::Utc::now();
    cloned_design.updated_at = chrono::Utc::now();
    
    tracing::info!("Cloned design {} to new design {}", id, cloned_design.id);
    
    Ok(Json(ApiResponse::success(cloned_design)))
}

/// Run simulation on a design
pub async fn run_simulation(
    State(pool): State<SqlitePool>,
    Json(request): Json<RunSimulationRequest>,
) -> Result<Json<ApiResponse<Vec<SimulationResult>>>, StatusCode> {
    let config = request.config.unwrap_or_default();
    
    // In a real implementation, load design and run actual simulation
    let mut results = Vec::new();
    
    for scenario in request.scenarios {
        let result = SimulationResult {
            design_id: request.design_id,
            speed: 25.0 + (scenario.conditions.rider_power / 10.0),
            acceleration: scenario.conditions.rider_power / 100.0,
            stopping_distance: 15.0 - (scenario.conditions.gradient * 2.0),
            comfort_rating: 7.5,
            safety_score: 85.0,
            energy_efficiency: 45.0,
        };
        results.push(result);
    }
    
    tracing::info!("Completed {} simulation scenarios for design {}", 
                  results.len(), request.design_id);
    
    Ok(Json(ApiResponse::success(results)))
}

/// Get simulation result
pub async fn get_simulation_result(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SimulationResult>>, StatusCode> {
    // In a real implementation, load from database
    let result = SimulationResult {
        design_id: id,
        speed: 25.0,
        acceleration: 2.5,
        stopping_distance: 15.0,
        comfort_rating: 7.5,
        safety_score: 85.0,
        energy_efficiency: 45.0,
    };
    
    Ok(Json(ApiResponse::success(result)))
}

/// Get performance analysis
pub async fn get_performance_analysis(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PerformanceAnalysis>>, StatusCode> {
    let mut performance_metrics = HashMap::new();
    performance_metrics.insert("speed".to_string(), 25.0);
    performance_metrics.insert("acceleration".to_string(), 2.5);
    performance_metrics.insert("efficiency".to_string(), 85.0);
    performance_metrics.insert("comfort".to_string(), 7.5);
    
    let analysis = PerformanceAnalysis {
        design_id: id,
        overall_score: 78.5,
        performance_metrics,
        recommendations: vec![
            "Consider lighter wheels for better acceleration".to_string(),
            "Upgrade to carbon fiber frame for weight reduction".to_string(),
            "Optimize gear ratios for target terrain".to_string(),
        ],
        optimal_use_cases: vec![
            "Road cycling".to_string(),
            "Recreational riding".to_string(),
            "Light touring".to_string(),
        ],
    };
    
    Ok(Json(ApiResponse::success(analysis)))
}

/// Get available materials
pub async fn get_materials(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<Vec<Material>>>, StatusCode> {
    let materials = vec![
        Material::Steel,
        Material::Aluminum6061,
        Material::Aluminum7075,
        Material::CarbonFiber,
        Material::Titanium,
        Material::Magnesium,
    ];
    
    Ok(Json(ApiResponse::success(materials)))
}

/// Select materials based on criteria
pub async fn select_materials(
    State(pool): State<SqlitePool>,
    Json(request): Json<MaterialSelectionRequest>,
) -> Result<Json<ApiResponse<Vec<(Material, f64)>>>, StatusCode> {
    let mut comparison = MaterialComparison::new(request.criteria);
    
    // Evaluate common materials
    let materials_to_evaluate = vec![
        Material::Steel,
        Material::Aluminum6061,
        Material::Aluminum7075,
        Material::CarbonFiber,
        Material::Titanium,
    ];
    
    for material in materials_to_evaluate {
        if let Some(max_cost) = request.max_cost {
            if material.properties().cost_per_kg > max_cost {
                continue;
            }
        }
        
        if request.eco_friendly_only.unwrap_or(false) {
            if material.environmental_score() < 60.0 {
                continue;
            }
        }
        
        comparison.evaluate_material(material, request.application.clone());
    }
    
    let best_materials = comparison.get_best_materials(5);
    
    Ok(Json(ApiResponse::success(best_materials)))
}

/// Compare materials
pub async fn compare_materials(
    State(pool): State<SqlitePool>,
    Json(materials): Json<Vec<Material>>,
) -> Result<Json<ApiResponse<MaterialComparison>>, StatusCode> {
    let criteria = MaterialSelectionCriteria {
        weight_importance: 0.25,
        strength_importance: 0.25,
        cost_importance: 0.2,
        environmental_importance: 0.15,
        durability_importance: 0.15,
    };
    
    let mut comparison = MaterialComparison::new(criteria);
    
    for material in materials {
        comparison.evaluate_material(material, MaterialApplication::Frame);
    }
    
    Ok(Json(ApiResponse::success(comparison)))
}

/// Compare designs
pub async fn compare_designs(
    State(pool): State<SqlitePool>,
    Json(request): Json<CompareDesignsRequest>,
) -> Result<Json<ApiResponse<HashMap<Uuid, PerformanceAnalysis>>>, StatusCode> {
    let mut comparisons = HashMap::new();
    
    for design_id in request.design_ids {
        let mut performance_metrics = HashMap::new();
        performance_metrics.insert("weight".to_string(), 8.5);
        performance_metrics.insert("speed".to_string(), 25.0);
        performance_metrics.insert("cost".to_string(), 2500.0);
        
        let analysis = PerformanceAnalysis {
            design_id,
            overall_score: 80.0,
            performance_metrics,
            recommendations: vec!["Sample recommendation".to_string()],
            optimal_use_cases: vec!["Road cycling".to_string()],
        };
        
        comparisons.insert(design_id, analysis);
    }
    
    Ok(Json(ApiResponse::success(comparisons)))
}

/// Optimize design
pub async fn optimize_design(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<BicycleDesign>>, StatusCode> {
    // In a real implementation, run optimization algorithms
    let mut optimized_design = BicycleDesign::new("Optimized Design");
    optimized_design.id = id;
    optimized_design.target_weight = 7.5; // Optimized weight
    optimized_design.updated_at = chrono::Utc::now();
    
    tracing::info!("Optimized design {}", id);
    
    Ok(Json(ApiResponse::success(optimized_design)))
}

/// Get analytics summary
pub async fn get_analytics_summary(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<HashMap<String, serde_json::Value>>>, StatusCode> {
    let mut summary = HashMap::new();
    summary.insert("total_designs".to_string(), serde_json::Value::from(42));
    summary.insert("total_simulations".to_string(), serde_json::Value::from(156));
    summary.insert("avg_performance_score".to_string(), serde_json::Value::from(78.5));
    summary.insert("most_popular_material".to_string(), serde_json::Value::from("Carbon Fiber"));
    
    Ok(Json(ApiResponse::success(summary)))
}

/// Generate performance report
pub async fn generate_performance_report(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let report = "# Bicycle Performance Report\n\n\
        ## Summary\n\
        - Total designs analyzed: 42\n\
        - Average performance score: 78.5/100\n\
        - Most efficient design: Carbon Road Racer v2.1\n\n\
        ## Recommendations\n\
        1. Focus on weight reduction for climbing performance\n\
        2. Improve aerodynamics for speed gains\n\
        3. Consider material cost vs. performance trade-offs\n";
    
    Ok(Json(ApiResponse::success(report.to_string())))
}

/// Get system configuration
pub async fn get_system_config(
    State(pool): State<SqlitePool>,
) -> Result<Json<ApiResponse<BicycleSystemConfig>>, StatusCode> {
    let config = BicycleSystemConfig::default();
    Ok(Json(ApiResponse::success(config)))
}

/// Update system configuration
pub async fn update_system_config(
    State(pool): State<SqlitePool>,
    Json(config): Json<BicycleSystemConfig>,
) -> Result<Json<ApiResponse<BicycleSystemConfig>>, StatusCode> {
    // In a real implementation, save to database
    tracing::info!("Updated bicycle system configuration");
    Ok(Json(ApiResponse::success(config)))
}

/// Health check endpoint
pub async fn health_check() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("Bicycle Design System is healthy".to_string())))
}

/// Query parameters for listing endpoints
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub filter: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u32, page: u32, limit: u32) -> Self {
        let total_pages = (total as f32 / limit as f32).ceil() as u32;
        Self {
            items,
            total,
            page,
            limit,
            total_pages,
        }
    }
}
