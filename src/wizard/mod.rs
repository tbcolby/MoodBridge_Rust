use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use async_trait::async_trait;
use anyhow::Result;

pub mod steps;
pub mod handlers;
pub mod case_wizard;
pub mod project_wizard;
pub mod integration_wizard;

/// Represents the current state of a wizard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardState {
    pub id: String,
    pub wizard_type: WizardType,
    pub current_step: usize,
    pub total_steps: usize,
    pub data: HashMap<String, serde_json::Value>,
    pub is_complete: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Types of wizards available in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WizardType {
    CaseCreation,
    ProjectSetup,
    ClientOnboarding,
    DocumentGeneration,
    SalesforceIntegration,
    ReportBuilder,
    WorkflowAutomation,
}

/// Represents a single step in a wizard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardStep {
    pub step_number: usize,
    pub title: String,
    pub description: String,
    pub fields: Vec<WizardField>,
    pub validation_rules: Vec<ValidationRule>,
    pub is_optional: bool,
    pub next_step_condition: Option<StepCondition>,
}

/// Form field in a wizard step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub options: Option<Vec<FieldOption>>,
    pub validation: Vec<ValidationRule>,
    pub help_text: Option<String>,
    pub conditional_display: Option<ConditionalDisplay>,
}

/// Types of form fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    TextArea,
    Number,
    Email,
    Phone,
    Date,
    DateTime,
    Select,
    MultiSelect,
    Checkbox,
    Radio,
    File,
    Currency,
    Password,
    Hidden,
}

/// Options for select/radio fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

/// Validation rules for fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationType,
    pub value: Option<serde_json::Value>,
    pub message: String,
}

/// Types of validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    Required,
    MinLength,
    MaxLength,
    Pattern,
    Email,
    Phone,
    Numeric,
    DateRange,
    Custom,
}

/// Conditional display logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalDisplay {
    pub field: String,
    pub condition: ConditionType,
    pub value: serde_json::Value,
}

/// Condition types for logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    IsEmpty,
    IsNotEmpty,
}

/// Step navigation conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCondition {
    pub field: String,
    pub condition: ConditionType,
    pub value: serde_json::Value,
    pub next_step: usize,
}

/// Wizard creation request
#[derive(Debug, Deserialize)]
pub struct CreateWizardRequest {
    pub wizard_type: WizardType,
    pub initial_data: Option<HashMap<String, serde_json::Value>>,
}

/// Wizard step submission
#[derive(Debug, Deserialize)]
pub struct StepSubmission {
    pub wizard_id: String,
    pub step_data: HashMap<String, serde_json::Value>,
    pub action: StepAction,
}

/// Actions that can be taken on a step
#[derive(Debug, Deserialize)]
pub enum StepAction {
    Next,
    Previous,
    Save,
    Complete,
    Cancel,
}

/// Wizard response for API
#[derive(Debug, Serialize)]
pub struct WizardResponse {
    pub state: WizardState,
    pub current_step_config: WizardStep,
    pub navigation: WizardNavigation,
    pub errors: Vec<ValidationError>,
}

/// Navigation state
#[derive(Debug, Serialize)]
pub struct WizardNavigation {
    pub can_go_previous: bool,
    pub can_go_next: bool,
    pub can_complete: bool,
    pub progress_percentage: f32,
}

/// Validation error
#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_type: String,
}

/// Trait for wizard implementations
#[async_trait]
pub trait Wizard {
    /// Get the wizard type
    fn wizard_type(&self) -> WizardType;
    
    /// Get all steps for this wizard
    fn get_steps(&self) -> Vec<WizardStep>;
    
    /// Validate step data
    async fn validate_step(&self, step: usize, data: &HashMap<String, serde_json::Value>) -> Result<Vec<ValidationError>>;
    
    /// Process step completion
    async fn process_step(&self, state: &mut WizardState, step_data: HashMap<String, serde_json::Value>) -> Result<()>;
    
    /// Complete the wizard
    async fn complete_wizard(&self, state: &WizardState) -> Result<serde_json::Value>;
    
    /// Get dynamic step based on current state
    fn get_dynamic_step(&self, state: &WizardState, step: usize) -> Option<WizardStep> {
        None
    }
}

/// Wizard manager for handling wizard lifecycle
pub struct WizardManager {
    pub wizards: HashMap<WizardType, Box<dyn Wizard + Send + Sync>>,
    pub states: HashMap<String, WizardState>,
}

impl WizardManager {
    pub fn new() -> Self {
        Self {
            wizards: HashMap::new(),
            states: HashMap::new(),
        }
    }
    
    /// Register a wizard implementation
    pub fn register_wizard(&mut self, wizard: Box<dyn Wizard + Send + Sync>) {
        let wizard_type = wizard.wizard_type();
        self.wizards.insert(wizard_type, wizard);
    }
    
    /// Create a new wizard instance
    pub async fn create_wizard(&mut self, request: CreateWizardRequest) -> Result<WizardState> {
        let wizard = self.wizards.get(&request.wizard_type)
            .ok_or_else(|| anyhow::anyhow!("Wizard type not found"))?;
        
        let wizard_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        let state = WizardState {
            id: wizard_id.clone(),
            wizard_type: request.wizard_type,
            current_step: 0,
            total_steps: wizard.get_steps().len(),
            data: request.initial_data.unwrap_or_default(),
            is_complete: false,
            created_at: now,
            updated_at: now,
        };
        
        self.states.insert(wizard_id, state.clone());
        Ok(state)
    }
    
    /// Get wizard state
    pub fn get_wizard_state(&self, wizard_id: &str) -> Option<&WizardState> {
        self.states.get(wizard_id)
    }
    
    /// Submit step data
    pub async fn submit_step(&mut self, submission: StepSubmission) -> Result<WizardResponse> {
        // Create default step outside of borrow
        let default_step = WizardStep {
            step_number: 0,
            title: "Error".to_string(),
            description: "An error occurred".to_string(),
            fields: Vec::new(),
            validation_rules: Vec::new(),
            is_optional: false,
            next_step_condition: None,
        };
        
        let state = self.states.get_mut(&submission.wizard_id)
            .ok_or_else(|| anyhow::anyhow!("Wizard not found"))?;
        
        let wizard = self.wizards.get(&state.wizard_type)
            .ok_or_else(|| anyhow::anyhow!("Wizard implementation not found"))?;
        
        // Validate step data
        let errors = wizard.validate_step(state.current_step, &submission.step_data).await?;
        
        if !errors.is_empty() {
            let current_step_config = wizard.get_steps().get(state.current_step)
                .cloned()
                .unwrap_or_else(|| default_step.clone());
            
            let navigation = WizardNavigation {
                can_go_previous: state.current_step > 0,
                can_go_next: state.current_step < state.total_steps - 1,
                can_complete: state.current_step == state.total_steps - 1,
                progress_percentage: ((state.current_step + 1) as f32 / state.total_steps as f32) * 100.0,
            };
            
            return Ok(WizardResponse {
                state: state.clone(),
                current_step_config,
                navigation,
                errors,
            });
        }
        
        // Process step
        wizard.process_step(state, submission.step_data).await?;
        
        // Handle action
        match submission.action {
            StepAction::Next => {
                if state.current_step < state.total_steps - 1 {
                    state.current_step += 1;
                }
            },
            StepAction::Previous => {
                if state.current_step > 0 {
                    state.current_step -= 1;
                }
            },
            StepAction::Complete => {
                if state.current_step == state.total_steps - 1 {
                    wizard.complete_wizard(state).await?;
                    state.is_complete = true;
                }
            },
            StepAction::Save => {
                // Just save, don't navigate
            },
            StepAction::Cancel => {
                // Handle cancellation
                self.states.remove(&submission.wizard_id);
                return Err(anyhow::anyhow!("Wizard cancelled"));
            },
        }
        
        state.updated_at = chrono::Utc::now();
        
        let current_step_config = wizard.get_steps().get(state.current_step)
            .cloned()
            .unwrap_or_else(|| default_step.clone());
        
        let navigation = WizardNavigation {
            can_go_previous: state.current_step > 0,
            can_go_next: state.current_step < state.total_steps - 1,
            can_complete: state.current_step == state.total_steps - 1,
            progress_percentage: ((state.current_step + 1) as f32 / state.total_steps as f32) * 100.0,
        };
        
        Ok(WizardResponse {
            state: state.clone(),
            current_step_config,
            navigation,
            errors: Vec::new(),
        })
    }
    
    /// Calculate navigation options
    pub fn calculate_navigation(&self, state: &WizardState) -> WizardNavigation {
        WizardNavigation {
            can_go_previous: state.current_step > 0,
            can_go_next: state.current_step < state.total_steps - 1,
            can_complete: state.current_step == state.total_steps - 1,
            progress_percentage: ((state.current_step + 1) as f32 / state.total_steps as f32) * 100.0,
        }
    }
    
    /// Get default step for error cases
    pub fn get_default_step(&self) -> WizardStep {
        WizardStep {
            step_number: 0,
            title: "Error".to_string(),
            description: "An error occurred".to_string(),
            fields: Vec::new(),
            validation_rules: Vec::new(),
            is_optional: false,
            next_step_condition: None,
        }
    }
}

impl Default for WizardManager {
    fn default() -> Self {
        Self::new()
    }
}
