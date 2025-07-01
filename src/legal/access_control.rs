/// Access Control Module for MoodBridge_Rust
/// 
/// This module provides role-based access control for legal operations,
/// ensuring only authorized users can perform sensitive legal functions.

use crate::legal::LegalOperationType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

/// User role definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserRole {
    /// Licensed attorney with full legal system access
    Attorney,
    /// Paralegal with supervised access to legal tools
    Paralegal,
    /// Legal assistant with limited access
    LegalAssistant,
    /// IT administrator with system access but limited legal functions
    Administrator,
    /// Regular user with minimal access
    User,
    /// Guest with read-only access
    Guest,
}

/// Permission levels for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// Full access without restrictions
    Full,
    /// Supervised access requiring attorney oversight
    Supervised,
    /// Read-only access
    ReadOnly,
    /// No access
    Denied,
}

/// User information for access control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub role: UserRole,
    pub bar_number: Option<String>,
    pub jurisdiction: Option<String>,
    pub supervisor_id: Option<String>,
    pub permissions: HashSet<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub active: bool,
}

/// Access control manager
#[derive(Debug, Clone)]
pub struct AccessController {
    users: HashMap<String, User>,
    role_permissions: HashMap<UserRole, HashSet<LegalOperationType>>,
}

impl AccessController {
    pub fn new() -> Self {
        let mut controller = Self {
            users: HashMap::new(),
            role_permissions: HashMap::new(),
        };
        
        controller.initialize_default_permissions();
        controller
    }

    /// Initialize default role-based permissions
    fn initialize_default_permissions(&mut self) {
        // Attorney permissions - full access to all operations
        let attorney_permissions = [
            LegalOperationType::AILegalAdvice,
            LegalOperationType::VoiceRecording,
            LegalOperationType::DocumentModification,
            LegalOperationType::ClientDataProcessing,
            LegalOperationType::TimelineAnalysis,
            LegalOperationType::DocumentIntelligence,
            LegalOperationType::PresentationGeneration,
            LegalOperationType::SemanticSearch,
            LegalOperationType::CollaborationMetrics,
        ].iter().cloned().collect();

        // Paralegal permissions - supervised access
        let paralegal_permissions = [
            LegalOperationType::DocumentIntelligence,
            LegalOperationType::PresentationGeneration,
            LegalOperationType::SemanticSearch,
            LegalOperationType::CollaborationMetrics,
            LegalOperationType::TimelineAnalysis,
        ].iter().cloned().collect();

        // Legal Assistant permissions - limited access
        let assistant_permissions = [
            LegalOperationType::DocumentIntelligence,
            LegalOperationType::PresentationGeneration,
            LegalOperationType::SemanticSearch,
        ].iter().cloned().collect();

        // Administrator permissions - system functions only
        let admin_permissions = [
            LegalOperationType::CollaborationMetrics,
        ].iter().cloned().collect();

        // User permissions - very limited
        let user_permissions = [
            LegalOperationType::SemanticSearch,
        ].iter().cloned().collect();

        // Guest permissions - none
        let guest_permissions = HashSet::new();

        self.role_permissions.insert(UserRole::Attorney, attorney_permissions);
        self.role_permissions.insert(UserRole::Paralegal, paralegal_permissions);
        self.role_permissions.insert(UserRole::LegalAssistant, assistant_permissions);
        self.role_permissions.insert(UserRole::Administrator, admin_permissions);
        self.role_permissions.insert(UserRole::User, user_permissions);
        self.role_permissions.insert(UserRole::Guest, guest_permissions);
    }

    /// Add or update a user
    pub async fn add_user(&mut self, user: User) -> Result<(), String> {
        // Validate user data
        if user.email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if user.role == UserRole::Attorney && user.bar_number.is_none() {
            return Err("Attorneys must have a bar number".to_string());
        }

        self.users.insert(user.user_id.clone(), user);
        tracing::info!("User added to access control system");
        Ok(())
    }

    /// Check if a user has permission for an operation
    pub async fn has_permission(&self, user_id: &str, operation_type: &LegalOperationType) -> bool {
        if let Some(user) = self.users.get(user_id) {
            if !user.active {
                return false;
            }

            // Check role-based permissions
            if let Some(role_permissions) = self.role_permissions.get(&user.role) {
                if role_permissions.contains(operation_type) {
                    return true;
                }
            }

            // Check individual permissions
            let operation_key = format!("{:?}", operation_type);
            return user.permissions.contains(&operation_key);
        }

        false
    }

    /// Get permission level for a user and operation
    pub async fn get_permission_level(&self, user_id: &str, operation_type: &LegalOperationType) -> PermissionLevel {
        if let Some(user) = self.users.get(user_id) {
            if !user.active {
                return PermissionLevel::Denied;
            }

            match (&user.role, operation_type) {
                // Attorneys have full access
                (UserRole::Attorney, _) => PermissionLevel::Full,
                
                // Paralegals need supervision for sensitive operations
                (UserRole::Paralegal, LegalOperationType::AILegalAdvice) => PermissionLevel::Supervised,
                (UserRole::Paralegal, LegalOperationType::DocumentModification) => PermissionLevel::Supervised,
                (UserRole::Paralegal, LegalOperationType::ClientDataProcessing) => PermissionLevel::Supervised,
                (UserRole::Paralegal, _) => {
                    if self.has_permission(user_id, operation_type).await {
                        PermissionLevel::Full
                    } else {
                        PermissionLevel::Denied
                    }
                }
                
                // Legal assistants have limited access
                (UserRole::LegalAssistant, _) => {
                    if self.has_permission(user_id, operation_type).await {
                        PermissionLevel::ReadOnly
                    } else {
                        PermissionLevel::Denied
                    }
                }
                
                // Others have minimal access
                _ => {
                    if self.has_permission(user_id, operation_type).await {
                        PermissionLevel::ReadOnly
                    } else {
                        PermissionLevel::Denied
                    }
                }
            }
        } else {
            PermissionLevel::Denied
        }
    }

    /// Check if user requires supervision for an operation
    pub async fn requires_supervision(&self, user_id: &str, operation_type: &LegalOperationType) -> bool {
        if let Some(user) = self.users.get(user_id) {
            match user.role {
                UserRole::Attorney => false, // Attorneys don't need supervision
                UserRole::Paralegal => {
                    matches!(operation_type,
                        LegalOperationType::AILegalAdvice |
                        LegalOperationType::DocumentModification |
                        LegalOperationType::ClientDataProcessing
                    )
                }
                _ => true, // All other roles need supervision for sensitive operations
            }
        } else {
            true // Unknown users need supervision
        }
    }

    /// Get supervising attorney for a user
    pub async fn get_supervisor(&self, user_id: &str) -> Option<&User> {
        if let Some(user) = self.users.get(user_id) {
            if let Some(supervisor_id) = &user.supervisor_id {
                return self.users.get(supervisor_id);
            }
        }
        None
    }

    /// Validate attorney credentials
    pub async fn validate_attorney(&self, user_id: &str) -> Result<bool, String> {
        if let Some(user) = self.users.get(user_id) {
            if user.role != UserRole::Attorney {
                return Ok(false);
            }

            // Check bar number
            if user.bar_number.is_none() {
                return Err("Attorney must have a bar number".to_string());
            }

            // Check jurisdiction
            if user.jurisdiction.is_none() {
                return Err("Attorney must specify jurisdiction".to_string());
            }

            // In a real implementation, this would validate against bar records
            Ok(true)
        } else {
            Err("User not found".to_string())
        }
    }

    /// Generate access control report for audit purposes
    pub async fn generate_access_report(&self) -> String {
        let mut report = String::from("=== ACCESS CONTROL AUDIT REPORT ===\n\n");
        
        report.push_str(&format!("Report Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        // User summary
        report.push_str("USER SUMMARY:\n");
        for role in [UserRole::Attorney, UserRole::Paralegal, UserRole::LegalAssistant, 
                     UserRole::Administrator, UserRole::User, UserRole::Guest] {
            let count = self.users.values().filter(|u| u.role == role).count();
            report.push_str(&format!("  {:?}: {} users\n", role, count));
        }
        
        report.push_str("\nUSER DETAILS:\n");
        for user in self.users.values() {
            report.push_str(&format!(
                "  {} ({}): Role={:?}, Active={}, Bar#={:?}, Jurisdiction={:?}\n",
                user.user_id,
                user.email,
                user.role,
                user.active,
                user.bar_number,
                user.jurisdiction
            ));
        }
        
        report.push_str("\nROLE PERMISSIONS:\n");
        for (role, permissions) in &self.role_permissions {
            report.push_str(&format!("  {:?}:\n", role));
            for permission in permissions {
                report.push_str(&format!("    - {:?}\n", permission));
            }
        }
        
        report
    }

    /// Create authentication token (simplified for demo)
    pub async fn create_auth_token(&self, user_id: &str) -> Result<String, String> {
        if let Some(user) = self.users.get(user_id) {
            if !user.active {
                return Err("User account is inactive".to_string());
            }
            
            // In a real implementation, this would create a proper JWT or similar token
            let token = format!("auth_{}_{}", user_id, Utc::now().timestamp());
            Ok(token)
        } else {
            Err("User not found".to_string())
        }
    }

    /// Validate authentication token (simplified for demo)
    pub async fn validate_auth_token(&self, token: &str) -> Result<String, String> {
        // In a real implementation, this would properly validate a JWT or similar token
        if token.starts_with("auth_") {
            let parts: Vec<&str> = token.split('_').collect();
            if parts.len() >= 2 {
                let user_id = parts[1];
                if self.users.contains_key(user_id) {
                    return Ok(user_id.to_string());
                }
            }
        }
        Err("Invalid token".to_string())
    }

    /// Grant temporary permission to a user
    pub async fn grant_temporary_permission(
        &mut self,
        user_id: &str,
        operation_type: LegalOperationType,
        granted_by: &str,
    ) -> Result<(), String> {
        // Verify the granter has permission to grant
        if !self.has_permission(granted_by, &operation_type).await {
            return Err("Insufficient privileges to grant permission".to_string());
        }

        if let Some(user) = self.users.get_mut(user_id) {
            let operation_key = format!("{:?}", operation_type);
            user.permissions.insert(operation_key);
            tracing::info!("Temporary permission granted to user {} by {}", user_id, granted_by);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Revoke permission from a user
    pub async fn revoke_permission(
        &mut self,
        user_id: &str,
        operation_type: LegalOperationType,
        revoked_by: &str,
    ) -> Result<(), String> {
        // Verify the revoker has admin privileges
        if let Some(revoker) = self.users.get(revoked_by) {
            if !matches!(revoker.role, UserRole::Attorney | UserRole::Administrator) {
                return Err("Insufficient privileges to revoke permission".to_string());
            }
        } else {
            return Err("Revoker not found".to_string());
        }

        if let Some(user) = self.users.get_mut(user_id) {
            let operation_key = format!("{:?}", operation_type);
            user.permissions.remove(&operation_key);
            tracing::info!("Permission revoked from user {} by {}", user_id, revoked_by);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Get user by ID
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    /// Update user last login
    pub async fn update_last_login(&mut self, user_id: &str) {
        if let Some(user) = self.users.get_mut(user_id) {
            user.last_login = Some(Utc::now());
        }
    }

    /// Deactivate user account
    pub async fn deactivate_user(&mut self, user_id: &str, deactivated_by: &str) -> Result<(), String> {
        // Verify deactivator has admin privileges
        if let Some(deactivator) = self.users.get(deactivated_by) {
            if !matches!(deactivator.role, UserRole::Attorney | UserRole::Administrator) {
                return Err("Insufficient privileges to deactivate user".to_string());
            }
        } else {
            return Err("Deactivator not found".to_string());
        }

        if let Some(user) = self.users.get_mut(user_id) {
            user.active = false;
            tracing::warn!("User {} deactivated by {}", user_id, deactivated_by);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}
