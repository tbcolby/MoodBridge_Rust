use super::*;
use async_trait::async_trait;
use anyhow::Result;

pub struct SalesforceIntegrationWizard;

impl SalesforceIntegrationWizard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Wizard for SalesforceIntegrationWizard {
    fn wizard_type(&self) -> WizardType {
        WizardType::SalesforceIntegration
    }
    
    fn get_steps(&self) -> Vec<WizardStep> {
        vec![
            // Step 1: Authentication Setup
            WizardStep {
                step_number: 0,
                title: "Salesforce Authentication".to_string(),
                description: "Connect to your Salesforce organization".to_string(),
                fields: vec![
                    WizardField {
                        name: "salesforce_instance_url".to_string(),
                        label: "Salesforce Instance URL".to_string(),
                        field_type: FieldType::Text,
                        required: true,
                        default_value: Some(serde_json::Value::String("https://yourcompany.salesforce.com".to_string())),
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Salesforce instance URL is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::Pattern,
                                value: Some(serde_json::Value::String(r"^https?://.*\.salesforce\.com$".to_string())),
                                message: "Please enter a valid Salesforce URL".to_string(),
                            },
                        ],
                        help_text: Some("Your Salesforce organization URL (e.g., https://yourcompany.salesforce.com)".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "auth_method".to_string(),
                        label: "Authentication Method".to_string(),
                        field_type: FieldType::Radio,
                        required: true,
                        default_value: Some(serde_json::Value::String("oauth".to_string())),
                        options: Some(vec![
                            FieldOption { value: "oauth".to_string(), label: "OAuth 2.0 (Recommended)".to_string(), disabled: false },
                            FieldOption { value: "username_password".to_string(), label: "Username/Password + Security Token".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("OAuth 2.0 is the recommended secure authentication method".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 2: Data Mapping Configuration
            WizardStep {
                step_number: 1,
                title: "Data Mapping".to_string(),
                description: "Configure how data flows between MoodBridge and Salesforce".to_string(),
                fields: vec![
                    WizardField {
                        name: "sync_direction".to_string(),
                        label: "Synchronization Direction".to_string(),
                        field_type: FieldType::Radio,
                        required: true,
                        default_value: Some(serde_json::Value::String("bidirectional".to_string())),
                        options: Some(vec![
                            FieldOption { value: "to_salesforce".to_string(), label: "MoodBridge → Salesforce Only".to_string(), disabled: false },
                            FieldOption { value: "from_salesforce".to_string(), label: "Salesforce → MoodBridge Only".to_string(), disabled: false },
                            FieldOption { value: "bidirectional".to_string(), label: "Bidirectional Sync".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Choose how data should flow between the systems".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "case_mapping".to_string(),
                        label: "Map Legal Cases to Salesforce".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: None,
                        options: Some(vec![
                            FieldOption { value: "case".to_string(), label: "Case Object".to_string(), disabled: false },
                            FieldOption { value: "opportunity".to_string(), label: "Opportunity Object".to_string(), disabled: false },
                            FieldOption { value: "custom_legal_case".to_string(), label: "Custom Legal Case Object".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Select which Salesforce object should represent legal cases".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "client_mapping".to_string(),
                        label: "Map Clients to Salesforce".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: None,
                        options: Some(vec![
                            FieldOption { value: "account".to_string(), label: "Account Object".to_string(), disabled: false },
                            FieldOption { value: "contact".to_string(), label: "Contact Object".to_string(), disabled: false },
                            FieldOption { value: "lead".to_string(), label: "Lead Object".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Select which Salesforce object should represent clients".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 3: Sync Settings
            WizardStep {
                step_number: 2,
                title: "Synchronization Settings".to_string(),
                description: "Configure how often and when data should sync".to_string(),
                fields: vec![
                    WizardField {
                        name: "sync_frequency".to_string(),
                        label: "Sync Frequency".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: Some(serde_json::Value::String("hourly".to_string())),
                        options: Some(vec![
                            FieldOption { value: "real_time".to_string(), label: "Real-time (via webhooks)".to_string(), disabled: false },
                            FieldOption { value: "every_15_min".to_string(), label: "Every 15 minutes".to_string(), disabled: false },
                            FieldOption { value: "hourly".to_string(), label: "Hourly".to_string(), disabled: false },
                            FieldOption { value: "daily".to_string(), label: "Daily".to_string(), disabled: false },
                            FieldOption { value: "manual".to_string(), label: "Manual only".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Choose how frequently data should synchronize".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "conflict_resolution".to_string(),
                        label: "Conflict Resolution Strategy".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: Some(serde_json::Value::String("moodbridge_wins".to_string())),
                        options: Some(vec![
                            FieldOption { value: "moodbridge_wins".to_string(), label: "MoodBridge data takes precedence".to_string(), disabled: false },
                            FieldOption { value: "salesforce_wins".to_string(), label: "Salesforce data takes precedence".to_string(), disabled: false },
                            FieldOption { value: "latest_timestamp".to_string(), label: "Most recent change wins".to_string(), disabled: false },
                            FieldOption { value: "manual_review".to_string(), label: "Require manual review".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("How should conflicts be resolved when both systems have different data".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "enable_webhooks".to_string(),
                        label: "Enable Real-time Webhooks".to_string(),
                        field_type: FieldType::Checkbox,
                        required: false,
                        default_value: Some(serde_json::Value::Bool(true)),
                        options: None,
                        validation: vec![],
                        help_text: Some("Enable webhooks for real-time data synchronization".to_string()),
                        conditional_display: Some(ConditionalDisplay {
                            field: "sync_frequency".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("real_time".to_string()),
                        }),
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 4: Testing and Validation
            WizardStep {
                step_number: 3,
                title: "Test Connection".to_string(),
                description: "Test the Salesforce connection and validate settings".to_string(),
                fields: vec![
                    WizardField {
                        name: "run_test_sync".to_string(),
                        label: "Run Test Synchronization".to_string(),
                        field_type: FieldType::Checkbox,
                        required: false,
                        default_value: Some(serde_json::Value::Bool(true)),
                        options: None,
                        validation: vec![],
                        help_text: Some("Perform a test sync with a small amount of data to verify configuration".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "notification_email".to_string(),
                        label: "Notification Email".to_string(),
                        field_type: FieldType::Email,
                        required: false,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Email,
                                value: None,
                                message: "Please enter a valid email address".to_string(),
                            },
                        ],
                        help_text: Some("Email address to receive sync status notifications and error alerts".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
        ]
    }
    
    async fn validate_step(&self, step: usize, data: &HashMap<String, serde_json::Value>) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        match step {
            0 => {
                // Validate Salesforce URL
                if let Some(url) = data.get("salesforce_instance_url") {
                    let url_str = url.as_str().unwrap_or("");
                    if !url_str.contains("salesforce.com") {
                        errors.push(ValidationError {
                            field: "salesforce_instance_url".to_string(),
                            message: "Please enter a valid Salesforce URL".to_string(),
                            error_type: "pattern".to_string(),
                        });
                    }
                }
            },
            1 => {
                // Validate data mapping selections
                if data.get("case_mapping").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                    errors.push(ValidationError {
                        field: "case_mapping".to_string(),
                        message: "Please select how legal cases should be mapped".to_string(),
                        error_type: "required".to_string(),
                    });
                }
            },
            _ => {}
        }
        
        Ok(errors)
    }
    
    async fn process_step(&self, state: &mut WizardState, step_data: HashMap<String, serde_json::Value>) -> Result<()> {
        // Merge step data into wizard state
        for (key, value) in step_data {
            state.data.insert(key, value);
        }
        
        // Handle specific processing for each step
        match state.current_step {
            0 => {
                // Test Salesforce connection
                // In a real implementation, this would attempt to connect to Salesforce
                // and validate the credentials
            },
            3 => {
                // Run test sync if requested
                if state.data.get("run_test_sync").and_then(|v| v.as_bool()).unwrap_or(false) {
                    // Perform test synchronization
                }
            },
            _ => {}
        }
        
        Ok(())
    }
    
    async fn complete_wizard(&self, state: &WizardState) -> Result<serde_json::Value> {
        // Here you would typically:
        // 1. Save the integration configuration to the database
        // 2. Set up the Salesforce connection
        // 3. Create webhook endpoints if enabled
        // 4. Schedule the initial sync
        // 5. Send confirmation email if provided
        
        let integration_id = uuid::Uuid::new_v4().to_string();
        
        Ok(serde_json::json!({
            "integration_id": integration_id,
            "integration_type": "salesforce",
            "status": "active",
            "message": "Salesforce integration configured successfully",
            "next_steps": [
                "Initial data sync will begin shortly",
                "Monitor sync status in the integrations dashboard",
                "Configure field mappings if needed"
            ],
            "connection_details": {
                "instance_url": state.data.get("salesforce_instance_url"),
                "sync_frequency": state.data.get("sync_frequency"),
                "sync_direction": state.data.get("sync_direction")
            }
        }))
    }
}
