use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Donor {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Donation {
    pub id: Uuid,
    pub donor_id: Uuid,
    pub amount: f64,
    pub date: chrono::NaiveDate,
    pub campaign_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub goal_amount: f64,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volunteer {
    pub id: Uuid,
    pub donor_id: Uuid,
    pub skills: Vec<String>,
    pub availability: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub date: chrono::NaiveDate,
    pub location: String,
    pub attendees: Vec<Uuid>,
}
