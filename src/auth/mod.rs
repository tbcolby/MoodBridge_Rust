pub mod providers;
pub mod kms;
pub mod tokens;
pub mod middleware;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Unified authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub provider: AuthProvider,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Supported authentication providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
    Microsoft,
    Salesforce,
    Snowflake,
    GitHub,
    Auth0,
    Okta,
    Custom(String),
}

/// OAuth2 configuration for any provider
#[derive(Debug, Clone)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub additional_params: HashMap<String, String>,
}

/// Authentication errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Provider not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),
    #[error("KMS error: {0}")]
    KmsError(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Main authentication trait - implemented by all providers
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn get_auth_url(&self, state: &str) -> Result<String, AuthError>;
    async fn exchange_code(&self, code: &str, state: &str) -> Result<AuthResult, AuthError>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResult, AuthError>;
    async fn validate_token(&self, token: &str) -> Result<AuthResult, AuthError>;
    async fn revoke_token(&self, token: &str) -> Result<(), AuthError>;
}

/// Authentication manager - orchestrates multiple providers
pub struct AuthManager {
    providers: HashMap<String, Box<dyn AuthProvider>>,
    kms: Box<dyn kms::KmsProvider>,
    default_provider: Option<String>,
}

impl AuthManager {
    pub fn new(kms: Box<dyn kms::KmsProvider>) -> Self {
        Self {
            providers: HashMap::new(),
            kms,
            default_provider: None,
        }
    }

    /// Register an authentication provider
    pub fn register_provider(&mut self, name: String, provider: Box<dyn AuthProvider>) {
        self.providers.insert(name, provider);
    }

    /// Set the default authentication provider
    pub fn set_default_provider(&mut self, name: String) {
        self.default_provider = Some(name);
    }

    /// Get authentication URL for a specific provider
    pub async fn get_auth_url(&self, provider: &str, state: &str) -> Result<String, AuthError> {
        let provider = self.providers.get(provider)
            .ok_or_else(|| AuthError::ProviderNotConfigured(provider.to_string()))?;
        
        provider.get_auth_url(state).await
    }

    /// Exchange authorization code for tokens
    pub async fn exchange_code(&self, provider: &str, code: &str, state: &str) -> Result<AuthResult, AuthError> {
        let provider = self.providers.get(provider)
            .ok_or_else(|| AuthError::ProviderNotConfigured(provider.to_string()))?;
        
        provider.exchange_code(code, state).await
    }

    /// Validate and refresh tokens as needed
    pub async fn validate_and_refresh(&self, provider: &str, token: &str, refresh_token: Option<&str>) -> Result<AuthResult, AuthError> {
        let provider = self.providers.get(provider)
            .ok_or_else(|| AuthError::ProviderNotConfigured(provider.to_string()))?;
        
        // Try to validate current token first
        match provider.validate_token(token).await {
            Ok(result) => Ok(result),
            Err(_) => {
                // If validation fails, try to refresh
                if let Some(refresh_token) = refresh_token {
                    provider.refresh_token(refresh_token).await
                } else {
                    Err(AuthError::TokenValidationFailed("No refresh token available".to_string()))
                }
            }
        }
    }

    /// Get available providers
    pub fn get_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    /// Securely store credentials using KMS
    pub async fn store_credentials(&self, user_id: &str, auth_result: &AuthResult) -> Result<(), AuthError> {
        let key = format!("auth:{}:{}", auth_result.provider as u8, user_id);
        let data = serde_json::to_string(auth_result)?;
        
        self.kms.encrypt_and_store(&key, &data).await
            .map_err(|e| AuthError::KmsError(e.to_string()))
    }

    /// Retrieve credentials from KMS
    pub async fn retrieve_credentials(&self, user_id: &str, provider: AuthProvider) -> Result<AuthResult, AuthError> {
        let key = format!("auth:{}:{}", provider as u8, user_id);
        
        let data = self.kms.retrieve_and_decrypt(&key).await
            .map_err(|e| AuthError::KmsError(e.to_string()))?;
        
        serde_json::from_str(&data).map_err(AuthError::JsonError)
    }
}

/// Configuration builder for easy setup
pub struct AuthConfigBuilder {
    manager: AuthManager,
}

impl AuthConfigBuilder {
    pub fn new(kms: Box<dyn kms::KmsProvider>) -> Self {
        Self {
            manager: AuthManager::new(kms),
        }
    }

    /// Add Google OAuth2 provider
    pub fn with_google(mut self, client_id: String, client_secret: String) -> Self {
        let config = OAuth2Config {
            client_id,
            client_secret,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uri: "http://localhost:8000/auth/google/callback".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            additional_params: HashMap::new(),
        };
        
        let provider = Box::new(providers::GoogleProvider::new(config));
        self.manager.register_provider("google".to_string(), provider);
        self
    }

    /// Add Salesforce OAuth2 provider
    pub fn with_salesforce(mut self, client_id: String, client_secret: String, domain: Option<String>) -> Self {
        let base_url = domain.unwrap_or_else(|| "https://login.salesforce.com".to_string());
        
        let config = OAuth2Config {
            client_id,
            client_secret,
            auth_url: format!("{}/services/oauth2/authorize", base_url),
            token_url: format!("{}/services/oauth2/token", base_url),
            redirect_uri: "http://localhost:8000/auth/salesforce/callback".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            additional_params: HashMap::new(),
        };
        
        let provider = Box::new(providers::SalesforceProvider::new(config));
        self.manager.register_provider("salesforce".to_string(), provider);
        self
    }

    /// Add Snowflake OAuth2 provider
    pub fn with_snowflake(mut self, client_id: String, client_secret: String, account_url: String) -> Self {
        let config = OAuth2Config {
            client_id,
            client_secret,
            auth_url: format!("{}/oauth/authorize", account_url),
            token_url: format!("{}/oauth/token-request", account_url),
            redirect_uri: "http://localhost:8000/auth/snowflake/callback".to_string(),
            scopes: vec!["session:role-any".to_string()],
            additional_params: HashMap::new(),
        };
        
        let provider = Box::new(providers::SnowflakeProvider::new(config));
        self.manager.register_provider("snowflake".to_string(), provider);
        self
    }

    /// Add Microsoft/Azure AD provider
    pub fn with_microsoft(mut self, client_id: String, client_secret: String, tenant_id: Option<String>) -> Self {
        let tenant = tenant_id.unwrap_or_else(|| "common".to_string());
        
        let config = OAuth2Config {
            client_id,
            client_secret,
            auth_url: format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", tenant),
            token_url: format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant),
            redirect_uri: "http://localhost:8000/auth/microsoft/callback".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string(), "User.Read".to_string()],
            additional_params: HashMap::new(),
        };
        
        let provider = Box::new(providers::MicrosoftProvider::new(config));
        self.manager.register_provider("microsoft".to_string(), provider);
        self
    }

    /// Add GitHub OAuth2 provider
    pub fn with_github(mut self, client_id: String, client_secret: String) -> Self {
        let config = OAuth2Config {
            client_id,
            client_secret,
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            redirect_uri: "http://localhost:8000/auth/github/callback".to_string(),
            scopes: vec!["user:email".to_string(), "read:user".to_string()],
            additional_params: HashMap::new(),
        };
        
        let provider = Box::new(providers::GitHubProvider::new(config));
        self.manager.register_provider("github".to_string(), provider);
        self
    }

    /// Set default provider
    pub fn with_default_provider(mut self, provider: &str) -> Self {
        self.manager.set_default_provider(provider.to_string());
        self
    }

    /// Build the authentication manager
    pub fn build(self) -> AuthManager {
        self.manager
    }
}

/// Easy configuration from environment variables
pub async fn from_env() -> Result<AuthManager, AuthError> {
    let kms = kms::create_kms_from_env().await?;
    let mut builder = AuthConfigBuilder::new(kms);

    // Google OAuth2
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("GOOGLE_CLIENT_ID"),
        std::env::var("GOOGLE_CLIENT_SECRET")
    ) {
        builder = builder.with_google(client_id, client_secret);
    }

    // Salesforce OAuth2
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("SALESFORCE_CLIENT_ID"),
        std::env::var("SALESFORCE_CLIENT_SECRET")
    ) {
        let domain = std::env::var("SALESFORCE_DOMAIN").ok();
        builder = builder.with_salesforce(client_id, client_secret, domain);
    }

    // Snowflake OAuth2
    if let (Ok(client_id), Ok(client_secret), Ok(account_url)) = (
        std::env::var("SNOWFLAKE_CLIENT_ID"),
        std::env::var("SNOWFLAKE_CLIENT_SECRET"),
        std::env::var("SNOWFLAKE_ACCOUNT_URL")
    ) {
        builder = builder.with_snowflake(client_id, client_secret, account_url);
    }

    // Microsoft OAuth2
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("MICROSOFT_CLIENT_ID"),
        std::env::var("MICROSOFT_CLIENT_SECRET")
    ) {
        let tenant_id = std::env::var("MICROSOFT_TENANT_ID").ok();
        builder = builder.with_microsoft(client_id, client_secret, tenant_id);
    }

    // GitHub OAuth2
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("GITHUB_CLIENT_ID"),
        std::env::var("GITHUB_CLIENT_SECRET")
    ) {
        builder = builder.with_github(client_id, client_secret);
    }

    // Set default provider
    if let Ok(default) = std::env::var("DEFAULT_AUTH_PROVIDER") {
        builder = builder.with_default_provider(&default);
    }

    Ok(builder.build())
}
