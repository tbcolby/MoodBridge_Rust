//! # Bicycle Design Wizard
//!
//! This module provides a guided wizard interface for creating bicycle designs,
//! making the complex design process accessible to users.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::design::{BicycleDesign, BicycleType, RiderFit, FlexibilityLevel, RidingStyle};
use super::materials::{Material, MaterialApplication};

/// Wizard step definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStep {
    pub step_id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub next_step: Option<String>,
    pub previous_step: Option<String>,
}

/// Question types for wizard steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub question_type: QuestionType,
    pub required: bool,
    pub help_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    SingleChoice {
        options: Vec<ChoiceOption>,
    },
    MultipleChoice {
        options: Vec<ChoiceOption>,
        max_selections: Option<usize>,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        unit: String,
    },
    Text {
        max_length: Option<usize>,
    },
    Number {
        min: Option<f64>,
        max: Option<f64>,
        unit: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

/// User's answers to wizard questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardAnswer {
    pub question_id: String,
    pub answer: AnswerValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnswerValue {
    Text(String),
    Number(f64),
    SingleChoice(String),
    MultipleChoice(Vec<String>),
}

/// Complete wizard session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardSession {
    pub session_id: Uuid,
    pub current_step: String,
    pub answers: Vec<WizardAnswer>,
    pub generated_design: Option<BicycleDesign>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Bicycle design wizard
pub struct BicycleWizard {
    pub steps: Vec<WizardStep>,
}

impl BicycleWizard {
    pub fn new() -> Self {
        Self {
            steps: Self::create_default_steps(),
        }
    }

    fn create_default_steps() -> Vec<WizardStep> {
        vec![
            // Step 1: Basic Information
            WizardStep {
                step_id: "basic_info".to_string(),
                title: "Basic Information".to_string(),
                description: "Let's start with some basic information about your bicycle project.".to_string(),
                questions: vec![
                    Question {
                        id: "design_name".to_string(),
                        text: "What would you like to name your bicycle design?".to_string(),
                        question_type: QuestionType::Text {
                            max_length: Some(50),
                        },
                        required: true,
                        help_text: Some("Choose a descriptive name for your bicycle design".to_string()),
                    },
                    Question {
                        id: "bicycle_type".to_string(),
                        text: "What type of bicycle are you designing?".to_string(),
                        question_type: QuestionType::SingleChoice {
                            options: vec![
                                ChoiceOption {
                                    value: "Road".to_string(),
                                    label: "Road Bike".to_string(),
                                    description: Some("Lightweight bike for paved surfaces and speed".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "Mountain".to_string(),
                                    label: "Mountain Bike".to_string(),
                                    description: Some("Rugged bike for off-road trails and terrain".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "Gravel".to_string(),
                                    label: "Gravel Bike".to_string(),
                                    description: Some("Versatile bike for mixed terrain".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "Commuter".to_string(),
                                    label: "Commuter Bike".to_string(),
                                    description: Some("Practical bike for daily transportation".to_string()),
                                    image_url: None,
                                },
                            ],
                        },
                        required: true,
                        help_text: Some("Choose the primary intended use for your bicycle".to_string()),
                    },
                ],
                next_step: Some("rider_fit".to_string()),
                previous_step: None,
            },

            // Step 2: Rider Fit
            WizardStep {
                step_id: "rider_fit".to_string(),
                title: "Rider Measurements".to_string(),
                description: "Help us create a properly fitted bicycle for you.".to_string(),
                questions: vec![
                    Question {
                        id: "rider_height".to_string(),
                        text: "What is your height?".to_string(),
                        question_type: QuestionType::Number {
                            min: Some(140.0),
                            max: Some(220.0),
                            unit: Some("cm".to_string()),
                        },
                        required: true,
                        help_text: Some("Enter your height in centimeters".to_string()),
                    },
                    Question {
                        id: "inseam".to_string(),
                        text: "What is your inseam measurement?".to_string(),
                        question_type: QuestionType::Number {
                            min: Some(60.0),
                            max: Some(110.0),
                            unit: Some("cm".to_string()),
                        },
                        required: true,
                        help_text: Some("Measure from crotch to floor while standing".to_string()),
                    },
                    Question {
                        id: "flexibility".to_string(),
                        text: "How would you describe your flexibility?".to_string(),
                        question_type: QuestionType::SingleChoice {
                            options: vec![
                                ChoiceOption {
                                    value: "Low".to_string(),
                                    label: "Low".to_string(),
                                    description: Some("Limited flexibility, prefer upright position".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "Medium".to_string(),
                                    label: "Medium".to_string(),
                                    description: Some("Average flexibility, comfortable with moderate reach".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "High".to_string(),
                                    label: "High".to_string(),
                                    description: Some("Very flexible, comfortable in aggressive positions".to_string()),
                                    image_url: None,
                                },
                            ],
                        },
                        required: true,
                        help_text: Some("This affects the bike's geometry and riding position".to_string()),
                    },
                ],
                next_step: Some("performance_goals".to_string()),
                previous_step: Some("basic_info".to_string()),
            },

            // Step 3: Performance Goals
            WizardStep {
                step_id: "performance_goals".to_string(),
                title: "Performance Goals".to_string(),
                description: "Tell us about your performance priorities.".to_string(),
                questions: vec![
                    Question {
                        id: "target_weight".to_string(),
                        text: "What is your target weight for the bicycle?".to_string(),
                        question_type: QuestionType::Slider {
                            min: 6.0,
                            max: 20.0,
                            step: 0.5,
                            unit: "kg".to_string(),
                        },
                        required: false,
                        help_text: Some("Lighter bikes are faster but may cost more".to_string()),
                    },
                    Question {
                        id: "budget".to_string(),
                        text: "What is your target budget?".to_string(),
                        question_type: QuestionType::SingleChoice {
                            options: vec![
                                ChoiceOption {
                                    value: "budget".to_string(),
                                    label: "Budget ($500-$1500)".to_string(),
                                    description: Some("Cost-effective components and materials".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "mid_range".to_string(),
                                    label: "Mid-Range ($1500-$4000)".to_string(),
                                    description: Some("Good balance of performance and cost".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "high_end".to_string(),
                                    label: "High-End ($4000-$8000)".to_string(),
                                    description: Some("Premium components and materials".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "no_limit".to_string(),
                                    label: "No Budget Limit".to_string(),
                                    description: Some("Best possible performance regardless of cost".to_string()),
                                    image_url: None,
                                },
                            ],
                        },
                        required: true,
                        help_text: Some("This will influence material and component recommendations".to_string()),
                    },
                    Question {
                        id: "priorities".to_string(),
                        text: "What are your top priorities? (Select up to 3)".to_string(),
                        question_type: QuestionType::MultipleChoice {
                            options: vec![
                                ChoiceOption {
                                    value: "speed".to_string(),
                                    label: "Speed".to_string(),
                                    description: Some("Maximum velocity and acceleration".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "comfort".to_string(),
                                    label: "Comfort".to_string(),
                                    description: Some("Smooth, comfortable ride quality".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "durability".to_string(),
                                    label: "Durability".to_string(),
                                    description: Some("Long-lasting, reliable components".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "weight".to_string(),
                                    label: "Lightweight".to_string(),
                                    description: Some("Minimal weight for easier handling".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "eco_friendly".to_string(),
                                    label: "Eco-Friendly".to_string(),
                                    description: Some("Sustainable materials and manufacturing".to_string()),
                                    image_url: None,
                                },
                            ],
                            max_selections: Some(3),
                        },
                        required: true,
                        help_text: Some("Choose your most important factors for the design".to_string()),
                    },
                ],
                next_step: Some("material_preferences".to_string()),
                previous_step: Some("rider_fit".to_string()),
            },

            // Step 4: Material Preferences
            WizardStep {
                step_id: "material_preferences".to_string(),
                title: "Material Preferences".to_string(),
                description: "Select your preferred materials for different components.".to_string(),
                questions: vec![
                    Question {
                        id: "frame_material".to_string(),
                        text: "What frame material do you prefer?".to_string(),
                        question_type: QuestionType::SingleChoice {
                            options: vec![
                                ChoiceOption {
                                    value: "carbon_fiber".to_string(),
                                    label: "Carbon Fiber".to_string(),
                                    description: Some("Lightweight, stiff, expensive".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "aluminum".to_string(),
                                    label: "Aluminum".to_string(),
                                    description: Some("Good balance of weight, cost, and performance".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "titanium".to_string(),
                                    label: "Titanium".to_string(),
                                    description: Some("Durable, comfortable, premium price".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "steel".to_string(),
                                    label: "Steel".to_string(),
                                    description: Some("Classic, repairable, smooth ride".to_string()),
                                    image_url: None,
                                },
                                ChoiceOption {
                                    value: "recommend".to_string(),
                                    label: "Recommend for me".to_string(),
                                    description: Some("Let the system choose based on your requirements".to_string()),
                                    image_url: None,
                                },
                            ],
                        },
                        required: true,
                        help_text: Some("Frame material significantly affects weight, cost, and ride quality".to_string()),
                    },
                ],
                next_step: Some("summary".to_string()),
                previous_step: Some("performance_goals".to_string()),
            },

            // Step 5: Summary
            WizardStep {
                step_id: "summary".to_string(),
                title: "Design Summary".to_string(),
                description: "Review your specifications and generate your bicycle design.".to_string(),
                questions: vec![],
                next_step: None,
                previous_step: Some("material_preferences".to_string()),
            },
        ]
    }

    /// Start a new wizard session
    pub fn start_session(&self) -> WizardSession {
        WizardSession {
            session_id: Uuid::new_v4(),
            current_step: "basic_info".to_string(),
            answers: Vec::new(),
            generated_design: None,
            started_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    /// Get a specific step
    pub fn get_step(&self, step_id: &str) -> Option<&WizardStep> {
        self.steps.iter().find(|step| step.step_id == step_id)
    }

    /// Submit answers for a step and move to the next step
    pub fn submit_step(&self, session: &mut WizardSession, answers: Vec<WizardAnswer>) -> Result<Option<String>, String> {
        // Validate answers
        let current_step = self.get_step(&session.current_step)
            .ok_or("Invalid step")?;
        
        // Add answers to session
        for answer in answers {
            // Remove any existing answer for this question
            session.answers.retain(|a| a.question_id != answer.question_id);
            session.answers.push(answer);
        }

        session.updated_at = chrono::Utc::now();

        // Move to next step or generate design
        if let Some(next_step) = &current_step.next_step {
            session.current_step = next_step.clone();
            Ok(Some(next_step.clone()))
        } else {
            // Final step - generate the design
            let design = self.generate_design(session)?;
            session.generated_design = Some(design);
            Ok(None) // No next step
        }
    }

    /// Generate a bicycle design based on wizard answers
    fn generate_design(&self, session: &WizardSession) -> Result<BicycleDesign, String> {
        let mut design = BicycleDesign::new("Wizard Generated Design");

        // Apply answers to design
        for answer in &session.answers {
            match answer.question_id.as_str() {
                "design_name" => {
                    if let AnswerValue::Text(name) = &answer.answer {
                        design.name = name.clone();
                    }
                },
                "bicycle_type" => {
                    if let AnswerValue::SingleChoice(bike_type) = &answer.answer {
                        design.intended_use = match bike_type.as_str() {
                            "Road" => BicycleType::Road,
                            "Mountain" => BicycleType::Mountain,
                            "Gravel" => BicycleType::Gravel,
                            "Commuter" => BicycleType::Commuter,
                            _ => BicycleType::Road,
                        };
                    }
                },
                "rider_height" => {
                    if let AnswerValue::Number(height) = &answer.answer {
                        design.rider_fit.rider_height = *height;
                        // Adjust frame geometry based on height
                        self.adjust_geometry_for_height(&mut design, *height);
                    }
                },
                "inseam" => {
                    if let AnswerValue::Number(inseam) = &answer.answer {
                        design.rider_fit.inseam = *inseam;
                    }
                },
                "flexibility" => {
                    if let AnswerValue::SingleChoice(flex) = &answer.answer {
                        design.rider_fit.flexibility = match flex.as_str() {
                            "Low" => FlexibilityLevel::Low,
                            "Medium" => FlexibilityLevel::Medium,
                            "High" => FlexibilityLevel::High,
                            _ => FlexibilityLevel::Medium,
                        };
                    }
                },
                "target_weight" => {
                    if let AnswerValue::Number(weight) = &answer.answer {
                        design.target_weight = *weight;
                    }
                },
                "budget" => {
                    if let AnswerValue::SingleChoice(budget) = &answer.answer {
                        design.target_price = match budget.as_str() {
                            "budget" => 1000.0,
                            "mid_range" => 2750.0,
                            "high_end" => 6000.0,
                            "no_limit" => 15000.0,
                            _ => 2000.0,
                        };
                    }
                },
                "frame_material" => {
                    if let AnswerValue::SingleChoice(material) = &answer.answer {
                        design.frame.material = match material.as_str() {
                            "carbon_fiber" => Material::CarbonFiber,
                            "aluminum" => Material::Aluminum6061,
                            "titanium" => Material::Titanium,
                            "steel" => Material::Steel,
                            _ => Material::Aluminum6061, // Default recommendation
                        };
                    }
                },
                _ => {},
            }
        }

        Ok(design)
    }

    fn adjust_geometry_for_height(&self, design: &mut BicycleDesign, height: f64) {
        // Simple geometry scaling based on height
        let scale_factor = height / 175.0; // Normalize to 175cm average
        
        design.frame.geometry.seat_tube_length *= scale_factor;
        design.frame.geometry.top_tube_length *= scale_factor;
        design.frame.geometry.head_tube_length *= scale_factor;
        design.frame.geometry.stack *= scale_factor;
        design.frame.geometry.reach *= scale_factor;
    }

    /// Get progress percentage for a session
    pub fn get_progress(&self, session: &WizardSession) -> f64 {
        let current_step_index = self.steps.iter()
            .position(|step| step.step_id == session.current_step)
            .unwrap_or(0);
        
        (current_step_index as f64 / self.steps.len() as f64) * 100.0
    }
}
