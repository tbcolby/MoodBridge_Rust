use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

pub mod trails;
pub mod modules;
pub mod badges;
pub mod challenges;
pub mod playground;
pub mod community;

/// Represents a learning trail (collection of modules)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trail {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub level: TrailLevel,
    pub estimated_time: String,
    pub modules: Vec<String>, // Module IDs
    pub prerequisites: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Represents a module within a trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub trail_id: String,
    pub order: u32,
    pub estimated_time: String,
    pub units: Vec<Unit>,
    pub learning_objectives: Vec<String>,
    pub prerequisites: Vec<String>,
}

/// Represents a unit within a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub title: String,
    pub content_type: ContentType,
    pub content: UnitContent,
    pub order: u32,
    pub estimated_time: String,
    pub learning_objectives: Vec<String>,
}

/// Types of content in a unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Reading,
    InteractiveDemo,
    HandsOnChallenge,
    CodeExample,
    Quiz,
    Video,
    Playground,
}

/// Content of a unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitContent {
    pub text: Option<String>,
    pub html: Option<String>,
    pub code_examples: Vec<CodeExample>,
    pub interactive_elements: Vec<InteractiveElement>,
    pub quiz_questions: Vec<QuizQuestion>,
    pub challenge: Option<Challenge>,
}

/// Code example with syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub language: String,
    pub code: String,
    pub explanation: String,
    pub filename: Option<String>,
    pub line_highlight: Option<Vec<u32>>,
}

/// Interactive elements like buttons, forms, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveElement {
    pub element_type: InteractiveType,
    pub id: String,
    pub label: String,
    pub action: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Types of interactive elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractiveType {
    Button,
    CodeEditor,
    WizardDemo,
    ApiTester,
    ConfigBuilder,
    Terminal,
}

/// Quiz question for knowledge checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub question: String,
    pub question_type: QuestionType,
    pub options: Vec<QuizOption>,
    pub correct_answer: String,
    pub explanation: String,
    pub points: u32,
}

/// Types of quiz questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    FillInTheBlank,
    CodeCompletion,
    DragAndDrop,
}

/// Quiz option for multiple choice questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizOption {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

/// Hands-on challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub instructions: Vec<String>,
    pub starter_code: Option<String>,
    pub solution: Option<String>,
    pub validation_criteria: Vec<ValidationCriterion>,
    pub hints: Vec<String>,
}

/// Validation criterion for challenges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCriterion {
    pub id: String,
    pub description: String,
    pub validation_type: ValidationType,
    pub expected_value: serde_json::Value,
    pub points: u32,
}

/// Types of validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    CodeContains,
    OutputMatches,
    ApiResponse,
    FileExists,
    DatabaseState,
    CustomFunction,
}

/// Skill levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrailLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Badge earned by completing trails/modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub badge_type: BadgeType,
    pub requirements: Vec<BadgeRequirement>,
    pub points: u32,
}

/// Types of badges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BadgeType {
    Trail,
    Module,
    Challenge,
    Community,
    Special,
}

/// Requirements to earn a badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeRequirement {
    pub requirement_type: RequirementType,
    pub target_id: String,
    pub description: String,
}

/// Types of badge requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementType {
    CompleteTrail,
    CompleteModule,
    CompleteChallenge,
    EarnPoints,
    CommunityContribution,
}

/// User progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub trail_progress: HashMap<String, TrailProgress>,
    pub earned_badges: Vec<String>,
    pub total_points: u32,
    pub streak_days: u32,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Progress on a specific trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrailProgress {
    pub trail_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub current_module: Option<String>,
    pub current_unit: Option<String>,
    pub module_progress: HashMap<String, ModuleProgress>,
    pub completion_percentage: f32,
}

/// Progress on a specific module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleProgress {
    pub module_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub unit_progress: HashMap<String, UnitProgress>,
    pub quiz_scores: HashMap<String, u32>,
    pub challenge_completed: bool,
}

/// Progress on a specific unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitProgress {
    pub unit_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub time_spent: u32, // in seconds
    pub interactions: Vec<UserInteraction>,
}

/// User interaction tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub interaction_type: InteractionType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: HashMap<String, serde_json::Value>,
}

/// Types of user interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    PageView,
    ButtonClick,
    CodeExecution,
    QuizAnswer,
    ChallengeSubmission,
    BookmarkAdded,
    NoteAdded,
}

/// Trailhead learning management system
pub struct TrailheadManager {
    pub trails: HashMap<String, Trail>,
    pub modules: HashMap<String, Module>,
    pub badges: HashMap<String, Badge>,
    pub user_progress: HashMap<String, UserProgress>,
}

impl TrailheadManager {
    pub fn new() -> Self {
        Self {
            trails: HashMap::new(),
            modules: HashMap::new(),
            badges: HashMap::new(),
            user_progress: HashMap::new(),
        }
    }

    /// Register a new trail
    pub fn register_trail(&mut self, trail: Trail) {
        self.trails.insert(trail.id.clone(), trail);
    }

    /// Register a new module
    pub fn register_module(&mut self, module: Module) {
        self.modules.insert(module.id.clone(), module);
    }

    /// Register a new badge
    pub fn register_badge(&mut self, badge: Badge) {
        self.badges.insert(badge.id.clone(), badge);
    }

    /// Get user's progress on a trail
    pub fn get_trail_progress(&self, user_id: &str, trail_id: &str) -> Option<&TrailProgress> {
        self.user_progress
            .get(user_id)?
            .trail_progress
            .get(trail_id)
    }

    /// Start a trail for a user
    pub fn start_trail(&mut self, user_id: &str, trail_id: &str) -> Result<(), String> {
        let trail = self.trails.get(trail_id)
            .ok_or("Trail not found")?;

        let progress = self.user_progress
            .entry(user_id.to_string())
            .or_insert_with(|| UserProgress {
                user_id: user_id.to_string(),
                trail_progress: HashMap::new(),
                earned_badges: Vec::new(),
                total_points: 0,
                streak_days: 0,
                last_activity: chrono::Utc::now(),
            });

        let trail_progress = TrailProgress {
            trail_id: trail_id.to_string(),
            started_at: chrono::Utc::now(),
            completed_at: None,
            current_module: trail.modules.first().cloned(),
            current_unit: None,
            module_progress: HashMap::new(),
            completion_percentage: 0.0,
        };

        progress.trail_progress.insert(trail_id.to_string(), trail_progress);
        progress.last_activity = chrono::Utc::now();

        Ok(())
    }

    /// Complete a unit for a user
    pub fn complete_unit(&mut self, user_id: &str, trail_id: &str, module_id: &str, unit_id: &str) -> Result<(), String> {
        let progress = self.user_progress
            .get_mut(user_id)
            .ok_or("User progress not found")?;

        let trail_progress = progress.trail_progress
            .get_mut(trail_id)
            .ok_or("Trail progress not found")?;

        let module_progress = trail_progress.module_progress
            .entry(module_id.to_string())
            .or_insert_with(|| ModuleProgress {
                module_id: module_id.to_string(),
                started_at: chrono::Utc::now(),
                completed_at: None,
                unit_progress: HashMap::new(),
                quiz_scores: HashMap::new(),
                challenge_completed: false,
            });

        let unit_progress = UnitProgress {
            unit_id: unit_id.to_string(),
            started_at: chrono::Utc::now(),
            completed_at: Some(chrono::Utc::now()),
            time_spent: 0,
            interactions: Vec::new(),
        };

        module_progress.unit_progress.insert(unit_id.to_string(), unit_progress);
        progress.last_activity = chrono::Utc::now();

        // Update completion percentage
        self.update_completion_percentage(user_id, trail_id)?;

        Ok(())
    }

    /// Update completion percentage for a trail
    fn update_completion_percentage(&mut self, user_id: &str, trail_id: &str) -> Result<(), String> {
        let trail = self.trails.get(trail_id).ok_or("Trail not found")?;
        let progress = self.user_progress.get_mut(user_id).ok_or("User progress not found")?;
        let trail_progress = progress.trail_progress.get_mut(trail_id).ok_or("Trail progress not found")?;

        let mut total_units = 0;
        let mut completed_units = 0;

        for module_id in &trail.modules {
            if let Some(module) = self.modules.get(module_id) {
                total_units += module.units.len();
                
                if let Some(module_progress) = trail_progress.module_progress.get(module_id) {
                    completed_units += module_progress.unit_progress.len();
                }
            }
        }

        trail_progress.completion_percentage = if total_units > 0 {
            (completed_units as f32 / total_units as f32) * 100.0
        } else {
            0.0
        };

        Ok(())
    }

    /// Check if user has earned any new badges
    pub fn check_badge_eligibility(&mut self, user_id: &str) -> Vec<String> {
        let mut new_badges = Vec::new();
        
        if let Some(progress) = self.user_progress.get(user_id) {
            for (badge_id, badge) in &self.badges {
                if !progress.earned_badges.contains(badge_id) {
                    if self.meets_badge_requirements(progress, badge) {
                        new_badges.push(badge_id.clone());
                    }
                }
            }
        }

        // Award the badges
        if let Some(progress) = self.user_progress.get_mut(user_id) {
            for badge_id in &new_badges {
                progress.earned_badges.push(badge_id.clone());
                if let Some(badge) = self.badges.get(badge_id) {
                    progress.total_points += badge.points;
                }
            }
        }

        new_badges
    }

    /// Check if user meets badge requirements
    fn meets_badge_requirements(&self, progress: &UserProgress, badge: &Badge) -> bool {
        for requirement in &badge.requirements {
            match requirement.requirement_type {
                RequirementType::CompleteTrail => {
                    if let Some(trail_progress) = progress.trail_progress.get(&requirement.target_id) {
                        if trail_progress.completion_percentage < 100.0 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                },
                RequirementType::CompleteModule => {
                    // Check if module is completed across all trails
                    let mut module_completed = false;
                    for trail_progress in progress.trail_progress.values() {
                        if let Some(module_progress) = trail_progress.module_progress.get(&requirement.target_id) {
                            if module_progress.completed_at.is_some() {
                                module_completed = true;
                                break;
                            }
                        }
                    }
                    if !module_completed {
                        return false;
                    }
                },
                RequirementType::EarnPoints => {
                    if let Ok(required_points) = requirement.target_id.parse::<u32>() {
                        if progress.total_points < required_points {
                            return false;
                        }
                    }
                },
                _ => {
                    // Other requirement types can be implemented as needed
                }
            }
        }
        true
    }
}

impl Default for TrailheadManager {
    fn default() -> Self {
        Self::new()
    }
}
