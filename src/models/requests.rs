use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use uuid::Uuid;
use validator::Validate as ValidateTrait;
use validator::{Validate, ValidationError};

/// Strip all HTML tags from input string
fn strip_html(input: &str) -> String {
    static HTML_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = HTML_REGEX.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());
    regex.replace_all(input, "").to_string()
}

/// AI prompt request with comprehensive validation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AiPromptRequest {
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Prompt must be between 1 and 10000 characters"
    ))]
    pub prompt: String,

    #[validate(custom(function = "validate_input_type"))]
    pub input_type: InputType,

    #[validate(custom(function = "validate_style"))]
    pub style: Option<String>,

    pub require_citations: Option<bool>,

    #[validate(range(
        min = 100,
        max = 4000,
        message = "Max response length must be between 100 and 4000"
    ))]
    pub max_response_length: Option<u32>,

    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Voice,
    Structured,
    Visual,
    Contextual,
}

impl AiPromptRequest {
    /// Sanitize the prompt to prevent XSS and other injection attacks
    pub fn sanitize(&mut self) {
        self.prompt = strip_html(&self.prompt);

        if let Some(ref mut style) = self.style {
            *style = strip_html(style);
        }
    }

    /// Validate and sanitize the request
    pub fn validate_and_sanitize(&mut self) -> Result<(), validator::ValidationErrors> {
        self.sanitize();
        ValidateTrait::validate(self)
    }
}

/// User registration request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserRegistrationRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(
        min = 2,
        max = 50,
        message = "Name must be between 2 and 50 characters"
    ))]
    pub name: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,

    #[validate(custom(function = "validate_password_confirm"))]
    pub password_confirm: String,

    #[validate(length(max = 100, message = "Organization name too long"))]
    pub organization: Option<String>,

    #[validate(custom(function = "validate_role"))]
    pub role: Option<String>,

    pub terms_accepted: bool,
    pub privacy_accepted: bool,
}

impl UserRegistrationRequest {
    pub fn sanitize(&mut self) {
        self.email = strip_html(&self.email.trim().to_lowercase());
        self.name = strip_html(self.name.trim());

        if let Some(ref mut org) = self.organization {
            *org = strip_html(org.trim());
        }

        if let Some(ref mut role) = self.role {
            *role = strip_html(role.trim());
        }
    }
}

/// User login request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UserLoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,

    pub remember_me: Option<bool>,
    pub mfa_token: Option<String>,
}

impl UserLoginRequest {
    pub fn sanitize(&mut self) {
        self.email = strip_html(&self.email.trim().to_lowercase());

        if let Some(ref mut token) = self.mfa_token {
            *token = strip_html(token.trim());
        }
    }
}

/// Case creation request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateCaseRequest {
    #[validate(length(
        min = 3,
        max = 200,
        message = "Case title must be between 3 and 200 characters"
    ))]
    pub title: String,

    #[validate(length(max = 5000, message = "Description too long"))]
    pub description: Option<String>,

    #[validate(custom(function = "validate_case_type"))]
    pub case_type: String,

    #[validate(custom(function = "validate_priority"))]
    pub priority: CasePriority,

    pub client_name: Option<String>,
    pub client_email: Option<String>,

    #[validate(custom(function = "validate_date"))]
    pub due_date: Option<DateTime<Utc>>,

    pub tags: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CasePriority {
    Low,
    Medium,
    High,
    Critical,
}

impl CreateCaseRequest {
    pub fn sanitize(&mut self) {
        self.title = strip_html(self.title.trim());

        if let Some(ref mut desc) = self.description {
            *desc = strip_html(desc.trim());
        }

        self.case_type = strip_html(self.case_type.trim());

        if let Some(ref mut name) = self.client_name {
            *name = strip_html(name.trim());
        }

        if let Some(ref mut email) = self.client_email {
            *email = strip_html(&email.trim().to_lowercase());
        }

        if let Some(ref mut tags) = self.tags {
            *tags = tags.iter().map(|tag| strip_html(tag.trim())).collect();
        }
    }
}

/// Incident report request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct IncidentReportRequest {
    #[validate(length(min = 3, max = 200, message = "Incident title required"))]
    pub title: String,

    #[validate(length(
        min = 10,
        max = 5000,
        message = "Description must be between 10 and 5000 characters"
    ))]
    pub description: String,

    #[validate(custom(function = "validate_incident_type"))]
    pub incident_type: String,

    #[validate(custom(function = "validate_severity"))]
    pub severity: IncidentSeverity,

    pub occurred_at: DateTime<Utc>,
    pub reported_by: Option<String>,
    pub witnesses: Option<Vec<String>>,
    pub evidence_urls: Option<Vec<String>>,
    pub immediate_actions: Option<String>,
    pub follow_up_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

impl IncidentReportRequest {
    pub fn sanitize(&mut self) {
        self.title = strip_html(self.title.trim());
        self.description = strip_html(self.description.trim());
        self.incident_type = strip_html(self.incident_type.trim());

        if let Some(ref mut reporter) = self.reported_by {
            *reporter = strip_html(reporter.trim());
        }

        if let Some(ref mut witnesses) = self.witnesses {
            *witnesses = witnesses.iter().map(|w| strip_html(w.trim())).collect();
        }

        if let Some(ref mut actions) = self.immediate_actions {
            *actions = strip_html(actions.trim());
        }
    }
}

/// Search request with filters
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchRequest {
    #[validate(length(min = 1, max = 500, message = "Search query required"))]
    pub query: String,

    #[validate(custom(function = "validate_search_type"))]
    pub search_type: SearchType,

    #[validate(range(min = 1, max = 100, message = "Page size must be between 1 and 100"))]
    pub page_size: Option<u32>,

    #[validate(range(min = 0, message = "Page must be non-negative"))]
    pub page: Option<u32>,

    pub filters: Option<SearchFilters>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Cases,
    Incidents,
    Documents,
    Users,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchFilters {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub case_type: Option<String>,
    pub priority: Option<CasePriority>,
    pub status: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub tags: Option<Vec<String>>,
}

impl SearchRequest {
    pub fn sanitize(&mut self) {
        self.query = strip_html(self.query.trim());

        if let Some(ref mut sort_by) = self.sort_by {
            *sort_by = strip_html(sort_by.trim());
        }

        if let Some(ref mut filters) = self.filters {
            if let Some(ref mut case_type) = filters.case_type {
                *case_type = strip_html(case_type.trim());
            }

            if let Some(ref mut status) = filters.status {
                *status = strip_html(status.trim());
            }

            if let Some(ref mut tags) = filters.tags {
                *tags = tags.iter().map(|tag| strip_html(tag.trim())).collect();
            }
        }
    }
}

// Custom validation functions

fn validate_input_type(input_type: &InputType) -> Result<(), ValidationError> {
    match input_type {
        InputType::Text
        | InputType::Voice
        | InputType::Structured
        | InputType::Visual
        | InputType::Contextual => Ok(()),
    }
}

fn validate_style(style: &str) -> Result<(), ValidationError> {
    let valid_styles = ["professional", "conversational", "technical", "executive"];
    if valid_styles.contains(&style) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_style"))
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 12 {
        return Err(ValidationError::new("password_too_short"));
    }

    if password.len() > 128 {
        return Err(ValidationError::new("password_too_long"));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(ValidationError::new("password_complexity"));
    }

    // Check for common weak patterns
    let weak_patterns = ["123456", "qwerty", "admin"];
    for pattern in weak_patterns {
        if password.to_lowercase().contains(pattern) {
            return Err(ValidationError::new("password_too_common"));
        }
    }

    Ok(())
}

fn validate_password_confirm(password_confirm: &str) -> Result<(), ValidationError> {
    // This would be validated against the main password in the handler
    if password_confirm.is_empty() {
        return Err(ValidationError::new("password_confirm_required"));
    }
    Ok(())
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    let valid_roles = ["admin", "lawyer", "paralegal", "client", "viewer"];
    if valid_roles.contains(&role) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_role"))
    }
}

fn validate_case_type(case_type: &str) -> Result<(), ValidationError> {
    let valid_types = [
        "family_law",
        "criminal_law",
        "civil_law",
        "corporate_law",
        "immigration_law",
        "intellectual_property",
        "real_estate",
        "other",
    ];
    if valid_types.contains(&case_type) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_case_type"))
    }
}

fn validate_priority(priority: &CasePriority) -> Result<(), ValidationError> {
    match priority {
        CasePriority::Low | CasePriority::Medium | CasePriority::High | CasePriority::Critical => {
            Ok(())
        }
    }
}

fn validate_incident_type(incident_type: &str) -> Result<(), ValidationError> {
    let valid_types = [
        "placement_denial",
        "communication_issue",
        "deadline_missed",
        "compliance_violation",
        "data_breach",
        "system_error",
        "other",
    ];
    if valid_types.contains(&incident_type) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_incident_type"))
    }
}

fn validate_severity(severity: &IncidentSeverity) -> Result<(), ValidationError> {
    match severity {
        IncidentSeverity::Minor
        | IncidentSeverity::Moderate
        | IncidentSeverity::Major
        | IncidentSeverity::Critical => Ok(()),
    }
}

fn validate_search_type(search_type: &SearchType) -> Result<(), ValidationError> {
    match search_type {
        SearchType::Cases
        | SearchType::Incidents
        | SearchType::Documents
        | SearchType::Users
        | SearchType::All => Ok(()),
    }
}

fn validate_date(date: &DateTime<Utc>) -> Result<(), ValidationError> {
    let now = Utc::now();
    let max_future = now + chrono::Duration::days(365 * 5); // 5 years in future max

    if *date > max_future {
        return Err(ValidationError::new("date_too_far_future"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_prompt_validation() {
        let mut request = AiPromptRequest {
            prompt: "Test prompt".to_string(),
            input_type: InputType::Text,
            style: Some("professional".to_string()),
            require_citations: Some(false),
            max_response_length: Some(1000),
            context: None,
        };

        assert!(request.validate_and_sanitize().is_ok());
    }

    #[test]
    fn test_ai_prompt_validation_fails() {
        let mut request = AiPromptRequest {
            prompt: "".to_string(), // Empty prompt should fail
            input_type: InputType::Text,
            style: Some("invalid_style".to_string()), // Invalid style
            require_citations: Some(false),
            max_response_length: Some(50), // Too small
            context: None,
        };

        assert!(request.validate_and_sanitize().is_err());
    }

    #[test]
    fn test_password_validation() {
        let test_password = "ValidPassword123!";
        println!("Testing password: {}", test_password);
        println!("Length: {}", test_password.len());
        println!(
            "Has uppercase: {}",
            test_password.chars().any(|c| c.is_uppercase())
        );
        println!(
            "Has lowercase: {}",
            test_password.chars().any(|c| c.is_lowercase())
        );
        println!(
            "Has digit: {}",
            test_password.chars().any(|c| c.is_numeric())
        );
        println!(
            "Has special: {}",
            test_password
                .chars()
                .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        );

        let result = validate_password(test_password);
        println!("Validation result: {:?}", result);
        assert!(result.is_ok());

        assert!(validate_password("short").is_err());
        assert!(validate_password("NoNumbersOrSpecial").is_err());
        assert!(validate_password("password123!").is_err()); // Contains "password"
    }

    #[test]
    fn test_sanitization() {
        let mut request = AiPromptRequest {
            prompt: "<script>alert('xss')</script>Safe content".to_string(),
            input_type: InputType::Text,
            style: Some("<b>professional</b>".to_string()),
            require_citations: Some(false),
            max_response_length: Some(1000),
            context: None,
        };

        request.sanitize();

        // Should remove script tags but keep safe content
        assert!(!request.prompt.contains("<script>"));
        assert!(request.prompt.contains("Safe content"));
        assert_eq!(request.style.as_ref().unwrap(), "professional");
    }

    #[test]
    fn test_user_registration_sanitization() {
        let mut request = UserRegistrationRequest {
            email: "  TEST@EXAMPLE.COM  ".to_string(),
            name: "<script>alert('xss')</script>John Doe".to_string(),
            password: "ValidPassword123!".to_string(),
            password_confirm: "ValidPassword123!".to_string(),
            organization: Some("  <b>Test Org</b>  ".to_string()),
            role: Some("admin".to_string()),
            terms_accepted: true,
            privacy_accepted: true,
        };

        request.sanitize();

        assert_eq!(request.email, "test@example.com");
        assert!(!request.name.contains("<script>"));
        assert!(request.name.contains("John Doe"));
        assert_eq!(request.organization.as_ref().unwrap(), "Test Org");
    }
}
