use config::{Config, ConfigError, Environment as ConfigEnvironment, File};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::info;
use validator::Validate;

/// Application configuration following Odaseva enterprise standards
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub security: SecurityConfig,
    pub ai: AiConfig,
    pub monitoring: MonitoringConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub keep_alive: u64,
    pub max_connections: usize,
    pub timeout_seconds: u64,
    pub shutdown_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub encryption_key: String,
    pub backup_enabled: bool,
    pub backup_interval_hours: u64,
    pub backup_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub refresh_token_expiry_days: u64,
    pub password_min_length: usize,
    pub password_require_special: bool,
    pub max_login_attempts: u32,
    pub lockout_duration_minutes: u32,
    pub session_timeout_minutes: u32,
    pub require_mfa: bool,
    pub cors_origins: Vec<String>,
    pub rate_limit_requests: u32,
    pub rate_limit_window_seconds: u64,
    pub https_only: bool,
    pub secure_headers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_seconds: u64,
    pub rate_limit_per_minute: u32,
    pub cache_enabled: bool,
    pub cache_ttl_seconds: u64,
    pub audit_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_enabled: bool,
    pub tracing_enabled: bool,
    pub sentry_dsn: Option<String>,
    pub prometheus_enabled: bool,
    pub alert_webhooks: Vec<String>,
    pub performance_threshold_ms: u64,
    pub error_rate_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String, // "json" or "text"
    pub file_enabled: bool,
    pub file_path: String,
    pub file_rotation: String, // "daily", "hourly", "size"
    pub max_file_size_mb: u64,
    pub retention_days: u32,
    pub console_enabled: bool,
    pub audit_log_enabled: bool,
    pub audit_log_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: None,
            keep_alive: 75,
            max_connections: 1000,
            timeout_seconds: 30,
            shutdown_timeout: 30,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://data/moodbridge.db?mode=rwc".to_string(),
            max_connections: 10,
            min_connections: 1,
            connection_timeout: 30,
            idle_timeout: 300,
            max_lifetime: 3600,
            encryption_key: generate_default_key(),
            backup_enabled: true,
            backup_interval_hours: 6,
            backup_retention_days: 30,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: generate_default_key(),
            jwt_expiry_hours: 24,
            refresh_token_expiry_days: 7,
            password_min_length: 12,
            password_require_special: true,
            max_login_attempts: 5,
            lockout_duration_minutes: 15,
            session_timeout_minutes: 60,
            require_mfa: false,
            cors_origins: vec!["http://localhost:3000".to_string()],
            rate_limit_requests: 100,
            rate_limit_window_seconds: 60,
            https_only: false, // Set to true in production
            secure_headers: true,
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: "openai".to_string(),
            api_key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            api_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: 2000,
            temperature: 0.3,
            timeout_seconds: 30,
            rate_limit_per_minute: 60,
            cache_enabled: true,
            cache_ttl_seconds: 3600,
            audit_enabled: true,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            metrics_port: 9090,
            health_check_enabled: true,
            tracing_enabled: true,
            sentry_dsn: env::var("SENTRY_DSN").ok(),
            prometheus_enabled: true,
            alert_webhooks: vec![],
            performance_threshold_ms: 1000,
            error_rate_threshold: 0.01, // 1%
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            file_enabled: true,
            file_path: "logs/app.log".to_string(),
            file_rotation: "daily".to_string(),
            max_file_size_mb: 100,
            retention_days: 30,
            console_enabled: true,
            audit_log_enabled: true,
            audit_log_path: "logs/audit.log".to_string(),
        }
    }
}

impl AppConfig {
    /// Load configuration from multiple sources with precedence:
    /// 1. Environment variables
    /// 2. config.yaml file
    /// 3. Default values
    pub fn load() -> Result<Self, ConfigError> {
        let mut settings = Config::builder();

        // Start with default values
        let default_config = AppConfig::default();

        // Add configuration file if it exists
        let config_file = env::var("CONFIG_FILE").unwrap_or_else(|_| "config.yaml".to_string());
        if std::path::Path::new(&config_file).exists() {
            settings = settings.add_source(File::with_name(&config_file));
            info!("Loaded configuration from file: {}", config_file);
        }

        // Add environment variables with MOODBRIDGE_ prefix
        settings = settings.add_source(
            ConfigEnvironment::with_prefix("MOODBRIDGE")
                .separator("_")
                .try_parsing(true),
        );

        let config = settings.build()?.try_deserialize::<AppConfig>()?;

        // Validate configuration
        config.validate()?;

        info!("Configuration loaded and validated successfully");
        Ok(config)
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate server configuration
        if self.server.port == 0 {
            return Err(ConfigError::Message("Server port cannot be 0".to_string()));
        }

        if self.server.max_connections == 0 {
            return Err(ConfigError::Message(
                "Max connections must be greater than 0".to_string(),
            ));
        }

        // Validate database configuration
        if self.database.url.is_empty() {
            return Err(ConfigError::Message(
                "Database URL cannot be empty".to_string(),
            ));
        }

        if self.database.max_connections == 0 {
            return Err(ConfigError::Message(
                "Database max connections must be greater than 0".to_string(),
            ));
        }

        // Validate security configuration
        if self.security.jwt_secret.len() < 32 {
            return Err(ConfigError::Message(
                "JWT secret must be at least 32 characters".to_string(),
            ));
        }

        if self.security.password_min_length < 8 {
            return Err(ConfigError::Message(
                "Password minimum length must be at least 8".to_string(),
            ));
        }

        // Validate AI configuration
        if self.ai.enabled && self.ai.api_key.is_empty() {
            return Err(ConfigError::Message(
                "AI API key is required when AI is enabled".to_string(),
            ));
        }

        if self.ai.max_tokens == 0 {
            return Err(ConfigError::Message(
                "AI max tokens must be greater than 0".to_string(),
            ));
        }

        // Validate monitoring configuration
        if self.monitoring.metrics_enabled && self.monitoring.metrics_port == 0 {
            return Err(ConfigError::Message(
                "Metrics port cannot be 0 when metrics are enabled".to_string(),
            ));
        }

        Ok(())
    }

    /// Get environment-specific configuration
    pub fn environment(&self) -> Environment {
        match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            "testing" => Environment::Testing,
            _ => Environment::Development,
        }
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        matches!(self.environment(), Environment::Production)
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        matches!(self.environment(), Environment::Development)
    }

    /// Get database connection string with encryption
    pub fn database_url(&self) -> String {
        if self.database.url.contains("?") {
            format!(
                "{}&encryption_key={}",
                self.database.url, self.database.encryption_key
            )
        } else {
            format!(
                "{}?encryption_key={}",
                self.database.url, self.database.encryption_key
            )
        }
    }

    /// Get server bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Get CORS origins for the environment
    pub fn cors_origins(&self) -> Vec<String> {
        if self.is_production() {
            // In production, only allow configured origins
            self.security.cors_origins.clone()
        } else {
            // In development, be more permissive
            let mut origins = self.security.cors_origins.clone();
            origins.extend_from_slice(&[
                "http://localhost:3000".to_string(),
                "http://localhost:3001".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ]);
            origins
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Testing => write!(f, "testing"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

/// Generate a secure default key for development
fn generate_default_key() -> String {
    use rand::Rng;

    // In production, this should be loaded from secure storage
    if env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        panic!("Default keys are not allowed in production. Set proper environment variables.");
    }

    let mut rng = rand::thread_rng();
    (0..64)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_port_validation() {
        let mut config = AppConfig::default();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_environment_detection() {
        let config = AppConfig::default();

        env::set_var("ENVIRONMENT", "production");
        assert!(config.is_production());

        env::set_var("ENVIRONMENT", "development");
        assert!(config.is_development());

        env::remove_var("ENVIRONMENT");
    }

    #[test]
    fn test_database_url_with_encryption() {
        let config = AppConfig::default();
        let url = config.database_url();
        assert!(url.contains("encryption_key="));
    }

    #[test]
    fn test_bind_address() {
        let config = AppConfig::default();
        assert_eq!(config.bind_address(), "127.0.0.1:8080");
    }
}
