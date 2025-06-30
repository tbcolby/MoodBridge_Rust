use crate::nonprofit::models::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct NonProfitService {
    donors: HashMap<Uuid, Donor>,
    donations: HashMap<Uuid, Donation>,
    campaigns: HashMap<Uuid, Campaign>,
    volunteers: HashMap<Uuid, Volunteer>,
    events: HashMap<Uuid, Event>,
}

impl NonProfitService {
    pub fn new() -> Self {
        Self {
            donors: HashMap::new(),
            donations: HashMap::new(),
            campaigns: HashMap::new(),
            volunteers: HashMap::new(),
            events: HashMap::new(),
        }
    }

    // Donor operations
    pub fn create_donor(&mut self, mut donor: Donor) -> Donor {
        donor.id = Uuid::new_v4();
        self.donors.insert(donor.id, donor.clone());
        donor
    }

    pub fn get_donors(&self) -> Vec<Donor> {
        self.donors.values().cloned().collect()
    }

    pub fn get_donor(&self, id: Uuid) -> Option<Donor> {
        self.donors.get(&id).cloned()
    }

    pub fn update_donor(&mut self, id: Uuid, updated_donor: Donor) -> Option<Donor> {
        if self.donors.contains_key(&id) {
            self.donors.insert(id, updated_donor.clone());
            Some(updated_donor)
        } else {
            None
        }
    }

    pub fn delete_donor(&mut self, id: Uuid) -> bool {
        self.donors.remove(&id).is_some()
    }

    // Donation operations
    pub fn create_donation(&mut self, mut donation: Donation) -> Donation {
        donation.id = Uuid::new_v4();
        self.donations.insert(donation.id, donation.clone());
        donation
    }

    pub fn get_donations(&self) -> Vec<Donation> {
        self.donations.values().cloned().collect()
    }

    pub fn get_donations_by_donor(&self, donor_id: Uuid) -> Vec<Donation> {
        self.donations
            .values()
            .filter(|d| d.donor_id == donor_id)
            .cloned()
            .collect()
    }

    pub fn get_donations_by_campaign(&self, campaign_id: Uuid) -> Vec<Donation> {
        self.donations
            .values()
            .filter(|d| d.campaign_id == Some(campaign_id))
            .cloned()
            .collect()
    }

    // Campaign operations
    pub fn create_campaign(&mut self, mut campaign: Campaign) -> Campaign {
        campaign.id = Uuid::new_v4();
        self.campaigns.insert(campaign.id, campaign.clone());
        campaign
    }

    pub fn get_campaigns(&self) -> Vec<Campaign> {
        self.campaigns.values().cloned().collect()
    }

    pub fn get_campaign(&self, id: Uuid) -> Option<Campaign> {
        self.campaigns.get(&id).cloned()
    }

    // Volunteer operations
    pub fn create_volunteer(&mut self, mut volunteer: Volunteer) -> Volunteer {
        volunteer.id = Uuid::new_v4();
        self.volunteers.insert(volunteer.id, volunteer.clone());
        volunteer
    }

    pub fn get_volunteers(&self) -> Vec<Volunteer> {
        self.volunteers.values().cloned().collect()
    }

    // Event operations
    pub fn create_event(&mut self, mut event: Event) -> Event {
        event.id = Uuid::new_v4();
        self.events.insert(event.id, event.clone());
        event
    }

    pub fn get_events(&self) -> Vec<Event> {
        self.events.values().cloned().collect()
    }

    // Analytics operations
    pub fn get_total_donations(&self) -> f64 {
        self.donations.values().map(|d| d.amount).sum()
    }

    pub fn get_campaign_progress(&self, campaign_id: Uuid) -> Option<f64> {
        let campaign = self.campaigns.get(&campaign_id)?;
        let total_raised: f64 = self
            .donations
            .values()
            .filter(|d| d.campaign_id == Some(campaign_id))
            .map(|d| d.amount)
            .sum();
        Some((total_raised / campaign.goal_amount) * 100.0)
    }

    pub fn get_donor_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_donors".to_string(), self.donors.len());
        stats.insert("total_donations".to_string(), self.donations.len());
        stats.insert("total_campaigns".to_string(), self.campaigns.len());
        stats.insert("total_volunteers".to_string(), self.volunteers.len());
        stats.insert("total_events".to_string(), self.events.len());
        stats
    }
}
