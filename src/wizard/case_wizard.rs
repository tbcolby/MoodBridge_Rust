use super::*;
use async_trait::async_trait;
use anyhow::Result;

pub struct CaseCreationWizard;

impl CaseCreationWizard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Wizard for CaseCreationWizard {
    fn wizard_type(&self) -> WizardType {
        WizardType::CaseCreation
    }
    
    fn get_steps(&self) -> Vec<WizardStep> {
        vec![
            // Step 1: Case Type Selection
            WizardStep {
                step_number: 0,
                title: "Case Type Selection".to_string(),
                description: "Select the type of legal case you want to create".to_string(),
                fields: vec![
                    WizardField {
                        name: "case_type".to_string(),
                        label: "Case Type".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: None,
                        options: Some(vec![
                            FieldOption { value: "family_law".to_string(), label: "Family Law".to_string(), disabled: false },
                            FieldOption { value: "criminal_defense".to_string(), label: "Criminal Defense".to_string(), disabled: false },
                            FieldOption { value: "personal_injury".to_string(), label: "Personal Injury".to_string(), disabled: false },
                            FieldOption { value: "business_law".to_string(), label: "Business Law".to_string(), disabled: false },
                            FieldOption { value: "real_estate".to_string(), label: "Real Estate".to_string(), disabled: false },
                            FieldOption { value: "intellectual_property".to_string(), label: "Intellectual Property".to_string(), disabled: false },
                        ]),
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Please select a case type".to_string(),
                            }
                        ],
                        help_text: Some("Choose the primary area of law for this case".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "case_subtype".to_string(),
                        label: "Case Subtype".to_string(),
                        field_type: FieldType::Select,
                        required: false,
                        default_value: None,
                        options: Some(vec![]), // Will be populated dynamically
                        validation: vec![],
                        help_text: Some("Select a more specific case category".to_string()),
                        conditional_display: Some(ConditionalDisplay {
                            field: "case_type".to_string(),
                            condition: ConditionType::NotEquals,
                            value: serde_json::Value::String("".to_string()),
                        }),
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 2: Client Information
            WizardStep {
                step_number: 1,
                title: "Client Information".to_string(),
                description: "Enter the primary client details for this case".to_string(),
                fields: vec![
                    WizardField {
                        name: "client_type".to_string(),
                        label: "Client Type".to_string(),
                        field_type: FieldType::Radio,
                        required: true,
                        default_value: Some(serde_json::Value::String("individual".to_string())),
                        options: Some(vec![
                            FieldOption { value: "individual".to_string(), label: "Individual".to_string(), disabled: false },
                            FieldOption { value: "business".to_string(), label: "Business/Corporation".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: None,
                        conditional_display: None,
                    },
                    WizardField {
                        name: "client_first_name".to_string(),
                        label: "First Name".to_string(),
                        field_type: FieldType::Text,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "First name is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::MinLength,
                                value: Some(serde_json::Value::Number(serde_json::Number::from(2))),
                                message: "First name must be at least 2 characters".to_string(),
                            },
                        ],
                        help_text: None,
                        conditional_display: Some(ConditionalDisplay {
                            field: "client_type".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("individual".to_string()),
                        }),
                    },
                    WizardField {
                        name: "client_last_name".to_string(),
                        label: "Last Name".to_string(),
                        field_type: FieldType::Text,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Last name is required".to_string(),
                            },
                        ],
                        help_text: None,
                        conditional_display: Some(ConditionalDisplay {
                            field: "client_type".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("individual".to_string()),
                        }),
                    },
                    WizardField {
                        name: "business_name".to_string(),
                        label: "Business Name".to_string(),
                        field_type: FieldType::Text,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Business name is required".to_string(),
                            },
                        ],
                        help_text: None,
                        conditional_display: Some(ConditionalDisplay {
                            field: "client_type".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("business".to_string()),
                        }),
                    },
                    WizardField {
                        name: "client_email".to_string(),
                        label: "Email Address".to_string(),
                        field_type: FieldType::Email,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Email address is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::Email,
                                value: None,
                                message: "Please enter a valid email address".to_string(),
                            },
                        ],
                        help_text: Some("Primary contact email for case communications".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "client_phone".to_string(),
                        label: "Phone Number".to_string(),
                        field_type: FieldType::Phone,
                        required: false,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Phone,
                                value: None,
                                message: "Please enter a valid phone number".to_string(),
                            },
                        ],
                        help_text: Some("Include area code for domestic numbers".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 3: Case Details
            WizardStep {
                step_number: 2,
                title: "Case Details".to_string(),
                description: "Provide specific details about the legal case".to_string(),
                fields: vec![
                    WizardField {
                        name: "case_title".to_string(),
                        label: "Case Title".to_string(),
                        field_type: FieldType::Text,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Case title is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::MinLength,
                                value: Some(serde_json::Value::Number(serde_json::Number::from(5))),
                                message: "Case title must be at least 5 characters".to_string(),
                            },
                        ],
                        help_text: Some("Brief descriptive title for the case".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "case_description".to_string(),
                        label: "Case Description".to_string(),
                        field_type: FieldType::TextArea,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Case description is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::MinLength,
                                value: Some(serde_json::Value::Number(serde_json::Number::from(20))),
                                message: "Please provide a more detailed description (at least 20 characters)".to_string(),
                            },
                        ],
                        help_text: Some("Detailed description of the legal matter, key facts, and issues involved".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "opposing_party".to_string(),
                        label: "Opposing Party".to_string(),
                        field_type: FieldType::Text,
                        required: false,
                        default_value: None,
                        options: None,
                        validation: vec![],
                        help_text: Some("Name of the opposing party or defendant (if applicable)".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "court_jurisdiction".to_string(),
                        label: "Court Jurisdiction".to_string(),
                        field_type: FieldType::Select,
                        required: false,
                        default_value: None,
                        options: Some(vec![
                            FieldOption { value: "federal".to_string(), label: "Federal Court".to_string(), disabled: false },
                            FieldOption { value: "state_superior".to_string(), label: "State Superior Court".to_string(), disabled: false },
                            FieldOption { value: "state_district".to_string(), label: "State District Court".to_string(), disabled: false },
                            FieldOption { value: "municipal".to_string(), label: "Municipal Court".to_string(), disabled: false },
                            FieldOption { value: "family".to_string(), label: "Family Court".to_string(), disabled: false },
                            FieldOption { value: "probate".to_string(), label: "Probate Court".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Select the appropriate court jurisdiction for this case".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "priority_level".to_string(),
                        label: "Priority Level".to_string(),
                        field_type: FieldType::Select,
                        required: true,
                        default_value: Some(serde_json::Value::String("medium".to_string())),
                        options: Some(vec![
                            FieldOption { value: "low".to_string(), label: "Low".to_string(), disabled: false },
                            FieldOption { value: "medium".to_string(), label: "Medium".to_string(), disabled: false },
                            FieldOption { value: "high".to_string(), label: "High".to_string(), disabled: false },
                            FieldOption { value: "urgent".to_string(), label: "Urgent".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: Some("Set the priority level for case management and scheduling".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 4: Financial Information
            WizardStep {
                step_number: 3,
                title: "Financial Information".to_string(),
                description: "Set up billing and financial arrangements for the case".to_string(),
                fields: vec![
                    WizardField {
                        name: "billing_type".to_string(),
                        label: "Billing Type".to_string(),
                        field_type: FieldType::Radio,
                        required: true,
                        default_value: Some(serde_json::Value::String("hourly".to_string())),
                        options: Some(vec![
                            FieldOption { value: "hourly".to_string(), label: "Hourly Rate".to_string(), disabled: false },
                            FieldOption { value: "flat_fee".to_string(), label: "Flat Fee".to_string(), disabled: false },
                            FieldOption { value: "contingency".to_string(), label: "Contingency".to_string(), disabled: false },
                            FieldOption { value: "retainer".to_string(), label: "Retainer".to_string(), disabled: false },
                        ]),
                        validation: vec![],
                        help_text: None,
                        conditional_display: None,
                    },
                    WizardField {
                        name: "hourly_rate".to_string(),
                        label: "Hourly Rate ($)".to_string(),
                        field_type: FieldType::Currency,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Hourly rate is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::Numeric,
                                value: None,
                                message: "Please enter a valid hourly rate".to_string(),
                            },
                        ],
                        help_text: Some("Standard hourly billing rate for this case".to_string()),
                        conditional_display: Some(ConditionalDisplay {
                            field: "billing_type".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("hourly".to_string()),
                        }),
                    },
                    WizardField {
                        name: "flat_fee_amount".to_string(),
                        label: "Flat Fee Amount ($)".to_string(),
                        field_type: FieldType::Currency,
                        required: true,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Required,
                                value: None,
                                message: "Flat fee amount is required".to_string(),
                            },
                            ValidationRule {
                                rule_type: ValidationType::Numeric,
                                value: None,
                                message: "Please enter a valid fee amount".to_string(),
                            },
                        ],
                        help_text: Some("Total flat fee for handling this case".to_string()),
                        conditional_display: Some(ConditionalDisplay {
                            field: "billing_type".to_string(),
                            condition: ConditionType::Equals,
                            value: serde_json::Value::String("flat_fee".to_string()),
                        }),
                    },
                    WizardField {
                        name: "retainer_amount".to_string(),
                        label: "Retainer Amount ($)".to_string(),
                        field_type: FieldType::Currency,
                        required: false,
                        default_value: None,
                        options: None,
                        validation: vec![
                            ValidationRule {
                                rule_type: ValidationType::Numeric,
                                value: None,
                                message: "Please enter a valid retainer amount".to_string(),
                            },
                        ],
                        help_text: Some("Initial retainer amount to be collected".to_string()),
                        conditional_display: None,
                    },
                ],
                validation_rules: vec![],
                is_optional: false,
                next_step_condition: None,
            },
            
            // Step 5: Review and Confirmation
            WizardStep {
                step_number: 4,
                title: "Review and Confirmation".to_string(),
                description: "Review all case information before creating the case".to_string(),
                fields: vec![
                    WizardField {
                        name: "create_initial_tasks".to_string(),
                        label: "Create Initial Tasks".to_string(),
                        field_type: FieldType::Checkbox,
                        required: false,
                        default_value: Some(serde_json::Value::Bool(true)),
                        options: None,
                        validation: vec![],
                        help_text: Some("Automatically create standard initial tasks for this case type".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "send_welcome_email".to_string(),
                        label: "Send Welcome Email to Client".to_string(),
                        field_type: FieldType::Checkbox,
                        required: false,
                        default_value: Some(serde_json::Value::Bool(true)),
                        options: None,
                        validation: vec![],
                        help_text: Some("Send an automated welcome email with case information to the client".to_string()),
                        conditional_display: None,
                    },
                    WizardField {
                        name: "schedule_initial_meeting".to_string(),
                        label: "Schedule Initial Client Meeting".to_string(),
                        field_type: FieldType::Checkbox,
                        required: false,
                        default_value: Some(serde_json::Value::Bool(false)),
                        options: None,
                        validation: vec![],
                        help_text: Some("Create a calendar event for the initial client consultation".to_string()),
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
                // Validate case type selection
                if let Some(case_type) = data.get("case_type") {
                    if case_type.as_str().unwrap_or("").is_empty() {
                        errors.push(ValidationError {
                            field: "case_type".to_string(),
                            message: "Please select a case type".to_string(),
                            error_type: "required".to_string(),
                        });
                    }
                }
            },
            1 => {
                // Validate client information
                if let Some(client_type) = data.get("client_type") {
                    let is_individual = client_type.as_str().unwrap_or("") == "individual";
                    
                    if is_individual {
                        if data.get("client_first_name").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                            errors.push(ValidationError {
                                field: "client_first_name".to_string(),
                                message: "First name is required for individual clients".to_string(),
                                error_type: "required".to_string(),
                            });
                        }
                        if data.get("client_last_name").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                            errors.push(ValidationError {
                                field: "client_last_name".to_string(),
                                message: "Last name is required for individual clients".to_string(),
                                error_type: "required".to_string(),
                            });
                        }
                    } else {
                        if data.get("business_name").and_then(|v| v.as_str()).unwrap_or("").is_empty() {
                            errors.push(ValidationError {
                                field: "business_name".to_string(),
                                message: "Business name is required for business clients".to_string(),
                                error_type: "required".to_string(),
                            });
                        }
                    }
                }
                
                // Validate email
                if let Some(email) = data.get("client_email") {
                    let email_str = email.as_str().unwrap_or("");
                    if email_str.is_empty() {
                        errors.push(ValidationError {
                            field: "client_email".to_string(),
                            message: "Email address is required".to_string(),
                            error_type: "required".to_string(),
                        });
                    } else if !email_str.contains('@') {
                        errors.push(ValidationError {
                            field: "client_email".to_string(),
                            message: "Please enter a valid email address".to_string(),
                            error_type: "email".to_string(),
                        });
                    }
                }
            },
            2 => {
                // Validate case details
                if data.get("case_title").and_then(|v| v.as_str()).unwrap_or("").len() < 5 {
                    errors.push(ValidationError {
                        field: "case_title".to_string(),
                        message: "Case title must be at least 5 characters".to_string(),
                        error_type: "min_length".to_string(),
                    });
                }
                
                if data.get("case_description").and_then(|v| v.as_str()).unwrap_or("").len() < 20 {
                    errors.push(ValidationError {
                        field: "case_description".to_string(),
                        message: "Please provide a more detailed description (at least 20 characters)".to_string(),
                        error_type: "min_length".to_string(),
                    });
                }
            },
            3 => {
                // Validate financial information
                if let Some(billing_type) = data.get("billing_type") {
                    match billing_type.as_str().unwrap_or("") {
                        "hourly" => {
                            if data.get("hourly_rate").and_then(|v| v.as_f64()).unwrap_or(0.0) <= 0.0 {
                                errors.push(ValidationError {
                                    field: "hourly_rate".to_string(),
                                    message: "Please enter a valid hourly rate greater than 0".to_string(),
                                    error_type: "numeric".to_string(),
                                });
                            }
                        },
                        "flat_fee" => {
                            if data.get("flat_fee_amount").and_then(|v| v.as_f64()).unwrap_or(0.0) <= 0.0 {
                                errors.push(ValidationError {
                                    field: "flat_fee_amount".to_string(),
                                    message: "Please enter a valid flat fee amount greater than 0".to_string(),
                                    error_type: "numeric".to_string(),
                                });
                            }
                        },
                        _ => {}
                    }
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
        
        // Handle dynamic field updates based on selections
        if let Some(case_type) = state.data.get("case_type") {
            self.update_subtypes(state, case_type.as_str().unwrap_or("")).await?;
        }
        
        Ok(())
    }
    
    async fn complete_wizard(&self, state: &WizardState) -> Result<serde_json::Value> {
        // Here you would typically:
        // 1. Create the case record in the database
        // 2. Set up initial tasks if requested
        // 3. Send welcome email if requested
        // 4. Schedule initial meeting if requested
        // 5. Return the created case information
        
        let case_id = uuid::Uuid::new_v4().to_string();
        
        // Mock case creation response
        Ok(serde_json::json!({
            "case_id": case_id,
            "case_number": format!("CASE-{}", chrono::Utc::now().format("%Y%m%d")),
            "status": "created",
            "message": "Case created successfully",
            "next_steps": [
                "Review case details",
                "Upload initial documents",
                "Schedule client meeting"
            ]
        }))
    }
}

impl CaseCreationWizard {
    async fn update_subtypes(&self, state: &mut WizardState, case_type: &str) -> Result<()> {
        // Update case subtypes based on selected case type
        let subtypes = match case_type {
            "family_law" => vec![
                ("divorce", "Divorce"),
                ("child_custody", "Child Custody"),
                ("child_support", "Child Support"),
                ("adoption", "Adoption"),
                ("domestic_violence", "Domestic Violence"),
            ],
            "criminal_defense" => vec![
                ("dui", "DUI/DWI"),
                ("assault", "Assault"),
                ("theft", "Theft"),
                ("drug_charges", "Drug Charges"),
                ("white_collar", "White Collar Crime"),
            ],
            "personal_injury" => vec![
                ("auto_accident", "Auto Accident"),
                ("medical_malpractice", "Medical Malpractice"),
                ("slip_fall", "Slip and Fall"),
                ("product_liability", "Product Liability"),
                ("workplace_injury", "Workplace Injury"),
            ],
            _ => vec![],
        };
        
        // Store subtypes for dynamic field population
        let subtypes_json: Vec<serde_json::Value> = subtypes.into_iter()
            .map(|(value, label)| serde_json::json!({
                "value": value,
                "label": label,
                "disabled": false
            }))
            .collect();
        
        state.data.insert("available_subtypes".to_string(), serde_json::Value::Array(subtypes_json));
        
        Ok(())
    }
}
