use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;
use thiserror::Error;
use tracing::{error, warn};
use uuid::Uuid;

/// Application-wide error types following Odaseva enterprise standards
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {message}")]
    Database {
        message: String,
        #[source]
        source: Option<sqlx::Error>,
    },

    #[error("Authentication failed: {reason}")]
    Authentication { reason: String },

    #[error("Authorization denied: {reason}")]
    Authorization { reason: String },

    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },

    #[error("AI processing error: {message}")]
    AiProcessing { message: String },

    #[error("Rate limit exceeded: {limit} requests per {window}")]
    RateLimit { limit: u32, window: String },

    #[error("Resource not found: {resource} with id {id}")]
    NotFound { resource: String, id: String },

    #[error("Conflict: {message}")]
    Conflict { message: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Internal server error: {message}")]
    Internal { message: String },

    #[error("Service unavailable: {message}")]
    ServiceUnavailable { message: String },
}

/// Error severity levels for monitoring and alerting
#[derive(Debug, Clone)]
pub enum ErrorSeverity {
    Low,      // Informational, expected errors
    Medium,   // User errors, validation failures
    High,     // Service errors, external failures
    Critical, // System failures, security issues
}

/// Extended error context for enterprise monitoring
#[derive(Debug)]
pub struct ErrorContext {
    pub error_id: Uuid,
    pub severity: ErrorSeverity,
    pub user_id: Option<String>,
    pub tenant_id: Option<String>,
    pub request_id: Option<String>,
    pub operation: Option<String>,
    pub additional_context: serde_json::Value,
}

impl AppError {
    /// Create a database error with detailed context
    pub fn database(message: impl Into<String>, source: Option<sqlx::Error>) -> Self {
        Self::Database {
            message: message.into(),
            source,
        }
    }

    /// Create an authentication error
    pub fn authentication(reason: impl Into<String>) -> Self {
        Self::Authentication {
            reason: reason.into(),
        }
    }

    /// Create an authorization error
    pub fn authorization(reason: impl Into<String>) -> Self {
        Self::Authorization {
            reason: reason.into(),
        }
    }

    /// Create a validation error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create an external service error
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
        }
    }

    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Authentication { .. } => StatusCode::UNAUTHORIZED,
            AppError::Authorization { .. } => StatusCode::FORBIDDEN,
            AppError::Validation { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Conflict { .. } => StatusCode::CONFLICT,
            AppError::RateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Database { .. }
            | AppError::ExternalService { .. }
            | AppError::AiProcessing { .. }
            | AppError::Configuration { .. }
            | AppError::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error severity for monitoring
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::Validation { .. } | AppError::NotFound { .. } => ErrorSeverity::Low,
            AppError::Authentication { .. }
            | AppError::Authorization { .. }
            | AppError::Conflict { .. }
            | AppError::RateLimit { .. } => ErrorSeverity::Medium,
            AppError::ExternalService { .. } | AppError::AiProcessing { .. } => ErrorSeverity::High,
            AppError::Database { .. }
            | AppError::Configuration { .. }
            | AppError::Internal { .. }
            | AppError::ServiceUnavailable { .. } => ErrorSeverity::Critical,
        }
    }

    /// Generate user-friendly error message (sanitized for public consumption)
    pub fn user_message(&self) -> &'static str {
        match self {
            AppError::Authentication { .. } => "Authentication required. Please log in.",
            AppError::Authorization { .. } => "You don't have permission to access this resource.",
            AppError::Validation { .. } => "The provided data is invalid. Please check your input.",
            AppError::NotFound { .. } => "The requested resource was not found.",
            AppError::Conflict { .. } => "This operation conflicts with existing data.",
            AppError::RateLimit { .. } => "Too many requests. Please try again later.",
            AppError::ServiceUnavailable { .. } => {
                "The service is temporarily unavailable. Please try again later."
            }
            _ => "An internal error occurred. Please try again or contact support.",
        }
    }

    /// Log error with appropriate level and context
    pub fn log_error(&self, context: &ErrorContext) {
        let error_details = json!({
            "error_id": context.error_id,
            "severity": format!("{:?}", context.severity),
            "user_id": context.user_id,
            "tenant_id": context.tenant_id,
            "request_id": context.request_id,
            "operation": context.operation,
            "error_type": self.to_string(),
            "additional_context": context.additional_context
        });

        match context.severity {
            ErrorSeverity::Low => {
                tracing::info!(error = %self, context = %error_details, "Application error occurred");
            }
            ErrorSeverity::Medium => {
                warn!(error = %self, context = %error_details, "User error occurred");
            }
            ErrorSeverity::High => {
                error!(error = %self, context = %error_details, "Service error occurred");
            }
            ErrorSeverity::Critical => {
                error!(error = %self, context = %error_details, "Critical system error occurred");
                // In production, this would trigger alerts
                #[cfg(feature = "sentry")]
                {
                    sentry::capture_error(self);
                }
            }
        }
    }
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            error_id: Uuid::new_v4(),
            severity: ErrorSeverity::Medium,
            user_id: None,
            tenant_id: None,
            request_id: None,
            operation: None,
            additional_context: json!({}),
        }
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        if let serde_json::Value::Object(ref mut map) = self.additional_context {
            map.insert(key.to_string(), value);
        }
        self
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let context = ErrorContext::new();
        self.log_error(&context);

        let status = self.status_code();
        let error_response = json!({
            "error": {
                "id": context.error_id,
                "type": "application_error",
                "status": status.as_u16(),
                "title": status.canonical_reason().unwrap_or("Unknown Error"),
                "detail": self.user_message(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        (status, Json(error_response)).into_response()
    }
}

// Implement From traits for common error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                resource: "record".to_string(),
                id: "unknown".to_string(),
            },
            sqlx::Error::Database(ref db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    AppError::Conflict {
                        message: format!("Database constraint violation: {}", constraint),
                    }
                } else {
                    let msg = db_err.message().to_string();
                    AppError::database(msg, Some(err))
                }
            }
            other => {
                let err_msg = other.to_string();
                AppError::database(err_msg, Some(other))
            }
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::validation("json", err.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        let field = err
            .field_errors()
            .keys()
            .next()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let message = err
            .field_errors()
            .values()
            .next()
            .and_then(|errors| errors.first())
            .and_then(|error| error.message.as_ref())
            .map(|msg| msg.to_string())
            .unwrap_or_else(|| "Validation failed".to_string());

        AppError::validation(field, message)
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

/// Macro for creating contextual errors
#[macro_export]
macro_rules! app_error {
    ($variant:ident, $($arg:expr),*) => {
        AppError::$variant { $($arg),* }
    };
}

/// Macro for creating contextual database errors
#[macro_export]
macro_rules! db_error {
    ($msg:expr) => {
        AppError::database($msg, None)
    };
    ($msg:expr, $source:expr) => {
        AppError::database($msg, Some($source))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            AppError::authentication("test").status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::authorization("test").status_code(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            AppError::validation("field", "message").status_code(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn test_error_severity() {
        assert!(matches!(
            AppError::validation("field", "message").severity(),
            ErrorSeverity::Low
        ));
        assert!(matches!(
            AppError::database("test", None).severity(),
            ErrorSeverity::Critical
        ));
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext::new()
            .with_user_id("user123")
            .with_operation("create_case");

        assert_eq!(context.user_id, Some("user123".to_string()));
        assert_eq!(context.operation, Some("create_case".to_string()));
    }
}
