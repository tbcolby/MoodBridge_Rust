use crate::nonprofit::{models::*, services::NonProfitService};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

type SharedService = Arc<Mutex<NonProfitService>>;

// Donor handlers
async fn create_donor(
    State(service): State<SharedService>,
    Json(donor): Json<Donor>,
) -> Result<Json<Donor>, StatusCode> {
    let mut service = service.lock().unwrap();
    let created_donor = service.create_donor(donor);
    Ok(Json(created_donor))
}

async fn get_donors(State(service): State<SharedService>) -> Result<Json<Vec<Donor>>, StatusCode> {
    let service = service.lock().unwrap();
    let donors = service.get_donors();
    Ok(Json(donors))
}

async fn get_donor(
    State(service): State<SharedService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Donor>, StatusCode> {
    let service = service.lock().unwrap();
    match service.get_donor(id) {
        Some(donor) => Ok(Json(donor)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Donation handlers
async fn create_donation(
    State(service): State<SharedService>,
    Json(donation): Json<Donation>,
) -> Result<Json<Donation>, StatusCode> {
    let mut service = service.lock().unwrap();
    let created_donation = service.create_donation(donation);
    Ok(Json(created_donation))
}

async fn get_donations(
    State(service): State<SharedService>,
) -> Result<Json<Vec<Donation>>, StatusCode> {
    let service = service.lock().unwrap();
    let donations = service.get_donations();
    Ok(Json(donations))
}

// Campaign handlers
async fn create_campaign(
    State(service): State<SharedService>,
    Json(campaign): Json<Campaign>,
) -> Result<Json<Campaign>, StatusCode> {
    let mut service = service.lock().unwrap();
    let created_campaign = service.create_campaign(campaign);
    Ok(Json(created_campaign))
}

async fn get_campaigns(
    State(service): State<SharedService>,
) -> Result<Json<Vec<Campaign>>, StatusCode> {
    let service = service.lock().unwrap();
    let campaigns = service.get_campaigns();
    Ok(Json(campaigns))
}

// Volunteer handlers
async fn create_volunteer(
    State(service): State<SharedService>,
    Json(volunteer): Json<Volunteer>,
) -> Result<Json<Volunteer>, StatusCode> {
    let mut service = service.lock().unwrap();
    let created_volunteer = service.create_volunteer(volunteer);
    Ok(Json(created_volunteer))
}

async fn get_volunteers(
    State(service): State<SharedService>,
) -> Result<Json<Vec<Volunteer>>, StatusCode> {
    let service = service.lock().unwrap();
    let volunteers = service.get_volunteers();
    Ok(Json(volunteers))
}

// Event handlers
async fn create_event(
    State(service): State<SharedService>,
    Json(event): Json<Event>,
) -> Result<Json<Event>, StatusCode> {
    let mut service = service.lock().unwrap();
    let created_event = service.create_event(event);
    Ok(Json(created_event))
}

async fn get_events(State(service): State<SharedService>) -> Result<Json<Vec<Event>>, StatusCode> {
    let service = service.lock().unwrap();
    let events = service.get_events();
    Ok(Json(events))
}

// Analytics endpoint
async fn get_analytics(
    State(service): State<SharedService>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let service = service.lock().unwrap();
    let stats = service.get_donor_stats();
    let total_donations = service.get_total_donations();

    let analytics = serde_json::json!({
        "total_donations_amount": total_donations,
        "statistics": stats
    });

    Ok(Json(analytics))
}

pub fn create_nonprofit_router() -> Router<SharedService> {
    let service = Arc::new(Mutex::new(NonProfitService::new()));

    Router::new()
        .route("/donors", post(create_donor).get(get_donors))
        .route("/donors/:id", get(get_donor))
        .route("/donations", post(create_donation).get(get_donations))
        .route("/campaigns", post(create_campaign).get(get_campaigns))
        .route("/volunteers", post(create_volunteer).get(get_volunteers))
        .route("/events", post(create_event).get(get_events))
        .route("/analytics", get(get_analytics))
        .with_state(service)
}
