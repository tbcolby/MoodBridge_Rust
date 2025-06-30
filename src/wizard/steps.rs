use super::*;

/// Utility functions for building common wizard steps
pub struct StepBuilder;

impl StepBuilder {
    /// Create a basic text input field
    pub fn text_field(name: &str, label: &str, required: bool, help_text: Option<&str>) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Text,
            required,
            default_value: None,
            options: None,
            validation: if required {
                vec![ValidationRule {
                    rule_type: ValidationType::Required,
                    value: None,
                    message: format!("{} is required", label),
                }]
            } else {
                vec![]
            },
            help_text: help_text.map(|s| s.to_string()),
            conditional_display: None,
        }
    }
    
    /// Create an email input field
    pub fn email_field(name: &str, label: &str, required: bool) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Email,
            required,
            default_value: None,
            options: None,
            validation: {
                let mut rules = vec![ValidationRule {
                    rule_type: ValidationType::Email,
                    value: None,
                    message: "Please enter a valid email address".to_string(),
                }];
                if required {
                    rules.push(ValidationRule {
                        rule_type: ValidationType::Required,
                        value: None,
                        message: format!("{} is required", label),
                    });
                }
                rules
            },
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a select dropdown field
    pub fn select_field(name: &str, label: &str, options: Vec<FieldOption>, required: bool) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Select,
            required,
            default_value: None,
            options: Some(options),
            validation: if required {
                vec![ValidationRule {
                    rule_type: ValidationType::Required,
                    value: None,
                    message: format!("Please select a {}", label.to_lowercase()),
                }]
            } else {
                vec![]
            },
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a radio button group
    pub fn radio_field(name: &str, label: &str, options: Vec<FieldOption>, default: Option<&str>) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Radio,
            required: true,
            default_value: default.map(|s| serde_json::Value::String(s.to_string())),
            options: Some(options),
            validation: vec![],
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a textarea field
    pub fn textarea_field(name: &str, label: &str, required: bool, min_length: Option<usize>) -> WizardField {
        let mut validation = vec![];
        
        if required {
            validation.push(ValidationRule {
                rule_type: ValidationType::Required,
                value: None,
                message: format!("{} is required", label),
            });
        }
        
        if let Some(min) = min_length {
            validation.push(ValidationRule {
                rule_type: ValidationType::MinLength,
                value: Some(serde_json::Value::Number(serde_json::Number::from(min))),
                message: format!("{} must be at least {} characters", label, min),
            });
        }
        
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::TextArea,
            required,
            default_value: None,
            options: None,
            validation,
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a currency/money field
    pub fn currency_field(name: &str, label: &str, required: bool) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Currency,
            required,
            default_value: None,
            options: None,
            validation: {
                let mut rules = vec![ValidationRule {
                    rule_type: ValidationType::Numeric,
                    value: None,
                    message: "Please enter a valid amount".to_string(),
                }];
                if required {
                    rules.push(ValidationRule {
                        rule_type: ValidationType::Required,
                        value: None,
                        message: format!("{} is required", label),
                    });
                }
                rules
            },
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a checkbox field
    pub fn checkbox_field(name: &str, label: &str, default_checked: bool, help_text: Option<&str>) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Checkbox,
            required: false,
            default_value: Some(serde_json::Value::Bool(default_checked)),
            options: None,
            validation: vec![],
            help_text: help_text.map(|s| s.to_string()),
            conditional_display: None,
        }
    }
    
    /// Create a date field
    pub fn date_field(name: &str, label: &str, required: bool) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Date,
            required,
            default_value: None,
            options: None,
            validation: if required {
                vec![ValidationRule {
                    rule_type: ValidationType::Required,
                    value: None,
                    message: format!("{} is required", label),
                }]
            } else {
                vec![]
            },
            help_text: None,
            conditional_display: None,
        }
    }
    
    /// Create a phone number field
    pub fn phone_field(name: &str, label: &str, required: bool) -> WizardField {
        WizardField {
            name: name.to_string(),
            label: label.to_string(),
            field_type: FieldType::Phone,
            required,
            default_value: None,
            options: None,
            validation: {
                let mut rules = vec![ValidationRule {
                    rule_type: ValidationType::Phone,
                    value: None,
                    message: "Please enter a valid phone number".to_string(),
                }];
                if required {
                    rules.push(ValidationRule {
                        rule_type: ValidationType::Required,
                        value: None,
                        message: format!("{} is required", label),
                    });
                }
                rules
            },
            help_text: Some("Include area code for domestic numbers".to_string()),
            conditional_display: None,
        }
    }
}

/// Common field option builders
pub struct FieldOptions;

impl FieldOptions {
    pub fn yes_no() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "yes".to_string(), label: "Yes".to_string(), disabled: false },
            FieldOption { value: "no".to_string(), label: "No".to_string(), disabled: false },
        ]
    }
    
    pub fn priority_levels() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "low".to_string(), label: "Low".to_string(), disabled: false },
            FieldOption { value: "medium".to_string(), label: "Medium".to_string(), disabled: false },
            FieldOption { value: "high".to_string(), label: "High".to_string(), disabled: false },
            FieldOption { value: "urgent".to_string(), label: "Urgent".to_string(), disabled: false },
        ]
    }
    
    pub fn case_types() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "family_law".to_string(), label: "Family Law".to_string(), disabled: false },
            FieldOption { value: "criminal_defense".to_string(), label: "Criminal Defense".to_string(), disabled: false },
            FieldOption { value: "personal_injury".to_string(), label: "Personal Injury".to_string(), disabled: false },
            FieldOption { value: "business_law".to_string(), label: "Business Law".to_string(), disabled: false },
            FieldOption { value: "real_estate".to_string(), label: "Real Estate".to_string(), disabled: false },
            FieldOption { value: "intellectual_property".to_string(), label: "Intellectual Property".to_string(), disabled: false },
        ]
    }
    
    pub fn client_types() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "individual".to_string(), label: "Individual".to_string(), disabled: false },
            FieldOption { value: "business".to_string(), label: "Business/Corporation".to_string(), disabled: false },
        ]
    }
    
    pub fn billing_types() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "hourly".to_string(), label: "Hourly Rate".to_string(), disabled: false },
            FieldOption { value: "flat_fee".to_string(), label: "Flat Fee".to_string(), disabled: false },
            FieldOption { value: "contingency".to_string(), label: "Contingency".to_string(), disabled: false },
            FieldOption { value: "retainer".to_string(), label: "Retainer".to_string(), disabled: false },
        ]
    }
    
    pub fn court_jurisdictions() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "federal".to_string(), label: "Federal Court".to_string(), disabled: false },
            FieldOption { value: "state_superior".to_string(), label: "State Superior Court".to_string(), disabled: false },
            FieldOption { value: "state_district".to_string(), label: "State District Court".to_string(), disabled: false },
            FieldOption { value: "municipal".to_string(), label: "Municipal Court".to_string(), disabled: false },
            FieldOption { value: "family".to_string(), label: "Family Court".to_string(), disabled: false },
            FieldOption { value: "probate".to_string(), label: "Probate Court".to_string(), disabled: false },
        ]
    }
    
    pub fn us_states() -> Vec<FieldOption> {
        vec![
            FieldOption { value: "AL".to_string(), label: "Alabama".to_string(), disabled: false },
            FieldOption { value: "AK".to_string(), label: "Alaska".to_string(), disabled: false },
            FieldOption { value: "AZ".to_string(), label: "Arizona".to_string(), disabled: false },
            FieldOption { value: "AR".to_string(), label: "Arkansas".to_string(), disabled: false },
            FieldOption { value: "CA".to_string(), label: "California".to_string(), disabled: false },
            FieldOption { value: "CO".to_string(), label: "Colorado".to_string(), disabled: false },
            FieldOption { value: "CT".to_string(), label: "Connecticut".to_string(), disabled: false },
            FieldOption { value: "DE".to_string(), label: "Delaware".to_string(), disabled: false },
            FieldOption { value: "FL".to_string(), label: "Florida".to_string(), disabled: false },
            FieldOption { value: "GA".to_string(), label: "Georgia".to_string(), disabled: false },
            // Add more states as needed...
        ]
    }
}

/// Validation helpers
pub struct ValidationHelpers;

impl ValidationHelpers {
    pub fn required_field(message: &str) -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::Required,
            value: None,
            message: message.to_string(),
        }
    }
    
    pub fn min_length(length: usize, message: &str) -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::MinLength,
            value: Some(serde_json::Value::Number(serde_json::Number::from(length))),
            message: message.to_string(),
        }
    }
    
    pub fn max_length(length: usize, message: &str) -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::MaxLength,
            value: Some(serde_json::Value::Number(serde_json::Number::from(length))),
            message: message.to_string(),
        }
    }
    
    pub fn email_format() -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::Email,
            value: None,
            message: "Please enter a valid email address".to_string(),
        }
    }
    
    pub fn phone_format() -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::Phone,
            value: None,
            message: "Please enter a valid phone number".to_string(),
        }
    }
    
    pub fn numeric_value() -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::Numeric,
            value: None,
            message: "Please enter a valid number".to_string(),
        }
    }
    
    pub fn pattern_match(pattern: &str, message: &str) -> ValidationRule {
        ValidationRule {
            rule_type: ValidationType::Pattern,
            value: Some(serde_json::Value::String(pattern.to_string())),
            message: message.to_string(),
        }
    }
}

/// Conditional display helpers
pub struct ConditionalHelpers;

impl ConditionalHelpers {
    pub fn show_when_equals(field: &str, value: &str) -> ConditionalDisplay {
        ConditionalDisplay {
            field: field.to_string(),
            condition: ConditionType::Equals,
            value: serde_json::Value::String(value.to_string()),
        }
    }
    
    pub fn hide_when_equals(field: &str, value: &str) -> ConditionalDisplay {
        ConditionalDisplay {
            field: field.to_string(),
            condition: ConditionType::NotEquals,
            value: serde_json::Value::String(value.to_string()),
        }
    }
    
    pub fn show_when_not_empty(field: &str) -> ConditionalDisplay {
        ConditionalDisplay {
            field: field.to_string(),
            condition: ConditionType::IsNotEmpty,
            value: serde_json::Value::Null,
        }
    }
    
    pub fn show_when_contains(field: &str, value: &str) -> ConditionalDisplay {
        ConditionalDisplay {
            field: field.to_string(),
            condition: ConditionType::Contains,
            value: serde_json::Value::String(value.to_string()),
        }
    }
}
