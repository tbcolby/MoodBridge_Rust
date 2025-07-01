/// Legal Compliance Module for MoodBridge_Rust
/// 
/// This module provides comprehensive legal compliance functionality including:
/// - Legal disclaimers and warnings
/// - User consent management
/// - Access control and authorization
/// - Audit logging for sensitive operations
/// - Professional responsibility safeguards

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub mod disclaimers;
pub mod consent;
pub mod access_control;
pub mod audit_log;
pub mod compliance_check;

/// Legal compliance status for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    RequiresConsent,
    RequiresAttorneyReview,
    Prohibited,
    ConditionallyAllowed,
}

/// Types of legal operations that require special handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegalOperationType {
    AILegalAdvice,
    VoiceRecording,
    DocumentModification,
    ClientDataProcessing,
    TimelineAnalysis,
    DocumentIntelligence,
    PresentationGeneration,
    SemanticSearch,
    CollaborationMetrics,
}

/// User consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub user_id: String,
    pub operation_type: LegalOperationType,
    pub consent_given: bool,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub consent_text: String,
    pub attorney_supervised: bool,
    pub disclaimer_acknowledged: bool,
}

/// Audit log entry for legal operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub entry_id: String,
    pub user_id: String,
    pub operation_type: LegalOperationType,
    pub timestamp: DateTime<Utc>,
    pub operation_details: HashMap<String, serde_json::Value>,
    pub compliance_status: ComplianceStatus,
    pub attorney_review_required: bool,
    pub data_processed: DataClassification,
}

/// Classification of data being processed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    PublicData,
    InternalUse,
    Confidential,
    AttorneyClientPrivileged,
    WorkProduct,
    PersonalIdentifiableInformation,
    HealthInformation,
}

/// Legal compliance result for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub allowed: bool,
    pub status: ComplianceStatus,
    pub required_disclaimers: Vec<String>,
    pub consent_required: bool,
    pub attorney_review_required: bool,
    pub audit_log_entry: Option<AuditLogEntry>,
    pub additional_requirements: Vec<String>,
}

/// Main legal compliance checker
pub struct LegalComplianceEngine {
    consent_manager: consent::ConsentManager,
    access_controller: access_control::AccessController,
    audit_logger: audit_log::AuditLogger,
}

impl LegalComplianceEngine {
    pub fn new() -> Self {
        Self {
            consent_manager: consent::ConsentManager::new(),
            access_controller: access_control::AccessController::new(),
            audit_logger: audit_log::AuditLogger::new(),
        }
    }

    /// Check if an operation is legally compliant
    pub async fn check_compliance(
        &self,
        user_id: &str,
        operation_type: LegalOperationType,
        data_classification: DataClassification,
        context: HashMap<String, serde_json::Value>,
    ) -> ComplianceResult {
        let mut result = ComplianceResult {
            allowed: false,
            status: ComplianceStatus::RequiresConsent,
            required_disclaimers: Vec::new(),
            consent_required: false,
            attorney_review_required: false,
            audit_log_entry: None,
            additional_requirements: Vec::new(),
        };

        // Check access permissions
        if !self.access_controller.has_permission(user_id, &operation_type).await {
            result.status = ComplianceStatus::Prohibited;
            result.additional_requirements.push("Insufficient access permissions".to_string());
            return result;
        }

        // Determine compliance requirements based on operation type
        match operation_type {
            LegalOperationType::AILegalAdvice => {
                result.required_disclaimers.push(disclaimers::AI_LEGAL_ADVICE_DISCLAIMER.to_string());
                result.attorney_review_required = true;
                result.consent_required = true;
                result.additional_requirements.push("Attorney supervision required".to_string());
            }
            LegalOperationType::VoiceRecording => {
                result.required_disclaimers.push(disclaimers::VOICE_RECORDING_DISCLAIMER.to_string());
                result.consent_required = true;
                result.additional_requirements.push("Recording consent required".to_string());
                
                // Check if this could involve privileged communications
                if matches!(data_classification, DataClassification::AttorneyClientPrivileged) {
                    result.attorney_review_required = true;
                    result.additional_requirements.push("Privilege protection review required".to_string());
                }
            }
            LegalOperationType::DocumentModification => {
                result.required_disclaimers.push(disclaimers::DOCUMENT_MODIFICATION_DISCLAIMER.to_string());
                result.attorney_review_required = true;
                result.consent_required = true;
                result.additional_requirements.push("Document integrity verification required".to_string());
                result.additional_requirements.push("Chain of custody documentation required".to_string());
            }
            LegalOperationType::ClientDataProcessing => {
                result.required_disclaimers.push(disclaimers::CLIENT_DATA_DISCLAIMER.to_string());
                result.consent_required = true;
                
                match data_classification {
                    DataClassification::AttorneyClientPrivileged => {
                        result.attorney_review_required = true;
                        result.additional_requirements.push("Privilege protection mandatory".to_string());
                    }
                    DataClassification::HealthInformation => {
                        result.additional_requirements.push("HIPAA compliance required".to_string());
                    }
                    DataClassification::PersonalIdentifiableInformation => {
                        result.additional_requirements.push("PII protection required".to_string());
                    }
                    _ => {}
                }
            }
            _ => {
                result.required_disclaimers.push(disclaimers::GENERAL_DISCLAIMER.to_string());
                result.consent_required = true;
            }
        }

        // Check consent status
        if result.consent_required {
            let consent_status = self.consent_manager
                .check_consent(user_id, &operation_type)
                .await;
            
            if !consent_status.consent_given {
                result.status = ComplianceStatus::RequiresConsent;
                result.allowed = false;
                return result;
            }
        }

        // If all checks pass, allow the operation
        result.allowed = true;
        result.status = if result.attorney_review_required {
            ComplianceStatus::RequiresAttorneyReview
        } else {
            ComplianceStatus::Compliant
        };

        // Create audit log entry
        let audit_entry = AuditLogEntry {
            entry_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            operation_type: operation_type.clone(),
            timestamp: Utc::now(),
            operation_details: context,
            compliance_status: result.status.clone(),
            attorney_review_required: result.attorney_review_required,
            data_processed: data_classification,
        };

        // Log the operation
        self.audit_logger.log_operation(&audit_entry).await;
        result.audit_log_entry = Some(audit_entry);

        result
    }

    /// Record user consent for an operation
    pub async fn record_consent(
        &mut self,
        user_id: &str,
        operation_type: LegalOperationType,
        consent_given: bool,
        ip_address: Option<String>,
        attorney_supervised: bool,
    ) -> Result<(), String> {
        let consent_record = ConsentRecord {
            user_id: user_id.to_string(),
            operation_type: operation_type.clone(),
            consent_given,
            timestamp: Utc::now(),
            ip_address,
            consent_text: self.get_consent_text(&operation_type),
            attorney_supervised,
            disclaimer_acknowledged: true,
        };

        self.consent_manager.record_consent(consent_record).await
    }

    /// Get appropriate consent text for an operation
    fn get_consent_text(&self, operation_type: &LegalOperationType) -> String {
        match operation_type {
            LegalOperationType::AILegalAdvice => {
                "I understand that this AI system does not provide legal advice and that I should consult with a qualified attorney for legal matters. I acknowledge that this system is for informational purposes only.".to_string()
            }
            LegalOperationType::VoiceRecording => {
                "I consent to the recording and processing of my voice for the purpose of this legal technology system. I understand that recordings may be stored and analyzed by AI systems.".to_string()
            }
            LegalOperationType::DocumentModification => {
                "I understand that modifying legal documents through this system may have legal consequences. I acknowledge that proper legal review and chain of custody procedures should be followed.".to_string()
            }
            _ => {
                "I acknowledge that I am using this legal technology system at my own discretion and understand the potential legal implications.".to_string()
            }
        }
    }
}

/// Helper function to create compliance wrapper for operations
pub async fn with_legal_compliance<F, T>(
    compliance_engine: &LegalComplianceEngine,
    user_id: &str,
    operation_type: LegalOperationType,
    data_classification: DataClassification,
    context: HashMap<String, serde_json::Value>,
    operation: F,
) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String>,
{
    let compliance_result = compliance_engine
        .check_compliance(user_id, operation_type, data_classification, context)
        .await;

    if !compliance_result.allowed {
        return Err(format!(
            "Operation not permitted: {:?}. Required disclaimers: {:?}. Additional requirements: {:?}",
            compliance_result.status,
            compliance_result.required_disclaimers,
            compliance_result.additional_requirements
        ));
    }

    if compliance_result.attorney_review_required {
        // In a real implementation, this would trigger attorney review workflow
        tracing::warn!("Attorney review required for this operation");
    }

    operation()
}
