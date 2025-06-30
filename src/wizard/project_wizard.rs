use super::*;
use async_trait::async_trait;
use anyhow::Result;

pub struct ProjectSetupWizard;

impl ProjectSetupWizard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Wizard for ProjectSetupWizard {
    fn wizard_type(&self) -> WizardType {
        WizardType::ProjectSetup
    }
    
    fn get_steps(&self) -> Vec<WizardStep> {
        // Placeholder implementation
        vec![
            WizardStep {
                step_number: 0,
                title: "Project Overview".to_string(),
                description: "Define the basic project information".to_string(),
                fields: vec![],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            }
        ]
    }
    
    async fn validate_step(&self, _step: usize, _data: &HashMap<String, serde_json::Value>) -> Result<Vec<ValidationError>> {
        Ok(vec![])
    }
    
    async fn process_step(&self, state: &mut WizardState, step_data: HashMap<String, serde_json::Value>) -> Result<()> {
        for (key, value) in step_data {
            state.data.insert(key, value);
        }
        Ok(())
    }
    
    async fn complete_wizard(&self, _state: &WizardState) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "project_id": uuid::Uuid::new_v4().to_string(),
            "status": "created",
            "message": "Project created successfully"
        }))
    }
}
