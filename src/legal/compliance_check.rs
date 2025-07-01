/// Compliance Checking Module for MoodBridge_Rust
/// 
/// This module provides automated compliance checking and validation rules
/// for legal operations, ensuring adherence to professional responsibility
/// and regulatory requirements.

use crate::legal::{LegalOperationType, DataClassification, ComplianceStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Compliance rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub rule_id: String,
    pub rule_name: String,
    pub description: String,
    pub operation_types: Vec<LegalOperationType>,
    pub data_classifications: Vec<DataClassification>,
    pub severity: RuleSeverity,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Severity levels for compliance rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

/// Compliance violation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub violation_id: String,
    pub rule_id: String,
    pub user_id: String,
    pub operation_type: LegalOperationType,
    pub data_classification: DataClassification,
    pub severity: RuleSeverity,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
    pub resolution_notes: Option<String>,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    pub compliant: bool,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub required_actions: Vec<String>,
}

/// Compliance checker engine
#[derive(Debug, Clone)]
pub struct ComplianceChecker {
    rules: HashMap<String, ComplianceRule>,
    violations: Vec<ComplianceViolation>,
}

impl ComplianceChecker {
    pub fn new() -> Self {
        let mut checker = Self {
            rules: HashMap::new(),
            violations: Vec::new(),
        };
        
        checker.initialize_default_rules();
        checker
    }

    /// Initialize default compliance rules
    fn initialize_default_rules(&mut self) {
        // Rule 1: Attorney-Client Privilege Protection
        self.add_rule(ComplianceRule {
            rule_id: "ACP001".to_string(),
            rule_name: "Attorney-Client Privilege Protection".to_string(),
            description: "Operations involving privileged communications must have attorney oversight".to_string(),
            operation_types: vec![
                LegalOperationType::AILegalAdvice,
                LegalOperationType::VoiceRecording,
                LegalOperationType::DocumentModification,
                LegalOperationType::ClientDataProcessing,
            ],
            data_classifications: vec![DataClassification::AttorneyClientPrivileged],
            severity: RuleSeverity::Critical,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 2: Unauthorized Practice of Law Prevention
        self.add_rule(ComplianceRule {
            rule_id: "UPL001".to_string(),
            rule_name: "Unauthorized Practice of Law Prevention".to_string(),
            description: "AI systems cannot provide legal advice without attorney supervision".to_string(),
            operation_types: vec![LegalOperationType::AILegalAdvice],
            data_classifications: vec![
                DataClassification::PublicData,
                DataClassification::InternalUse,
                DataClassification::Confidential,
            ],
            severity: RuleSeverity::Critical,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 3: Voice Recording Consent
        self.add_rule(ComplianceRule {
            rule_id: "VRC001".to_string(),
            rule_name: "Voice Recording Consent Requirement".to_string(),
            description: "Voice recordings require explicit consent from all parties".to_string(),
            operation_types: vec![LegalOperationType::VoiceRecording],
            data_classifications: vec![
                DataClassification::AttorneyClientPrivileged,
                DataClassification::Confidential,
                DataClassification::PersonalIdentifiableInformation,
            ],
            severity: RuleSeverity::Critical,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 4: Document Integrity Protection
        self.add_rule(ComplianceRule {
            rule_id: "DIP001".to_string(),
            rule_name: "Document Integrity Protection".to_string(),
            description: "Document modifications must maintain chain of custody".to_string(),
            operation_types: vec![LegalOperationType::DocumentModification],
            data_classifications: vec![
                DataClassification::AttorneyClientPrivileged,
                DataClassification::WorkProduct,
                DataClassification::Confidential,
            ],
            severity: RuleSeverity::High,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 5: HIPAA Compliance for Health Information
        self.add_rule(ComplianceRule {
            rule_id: "HIPAA001".to_string(),
            rule_name: "HIPAA Health Information Protection".to_string(),
            description: "Health information processing must comply with HIPAA requirements".to_string(),
            operation_types: vec![
                LegalOperationType::ClientDataProcessing,
                LegalOperationType::DocumentIntelligence,
                LegalOperationType::SemanticSearch,
            ],
            data_classifications: vec![DataClassification::HealthInformation],
            severity: RuleSeverity::Critical,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 6: PII Data Protection
        self.add_rule(ComplianceRule {
            rule_id: "PII001".to_string(),
            rule_name: "Personal Information Protection".to_string(),
            description: "Personal identifiable information requires special handling".to_string(),
            operation_types: vec![
                LegalOperationType::ClientDataProcessing,
                LegalOperationType::CollaborationMetrics,
                LegalOperationType::DocumentIntelligence,
            ],
            data_classifications: vec![DataClassification::PersonalIdentifiableInformation],
            severity: RuleSeverity::High,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        // Rule 7: Work Product Protection
        self.add_rule(ComplianceRule {
            rule_id: "WPP001".to_string(),
            rule_name: "Work Product Doctrine Protection".to_string(),
            description: "Attorney work product must be protected from disclosure".to_string(),
            operation_types: vec![
                LegalOperationType::TimelineAnalysis,
                LegalOperationType::DocumentIntelligence,
                LegalOperationType::PresentationGeneration,
            ],
            data_classifications: vec![DataClassification::WorkProduct],
            severity: RuleSeverity::High,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });
    }

    /// Add a compliance rule
    pub fn add_rule(&mut self, rule: ComplianceRule) {
        self.rules.insert(rule.rule_id.clone(), rule);
    }

    /// Perform comprehensive compliance check
    pub async fn check_compliance(
        &mut self,
        user_id: &str,
        operation_type: LegalOperationType,
        data_classification: DataClassification,
        context: &HashMap<String, serde_json::Value>,
    ) -> ComplianceCheckResult {
        let mut result = ComplianceCheckResult {
            compliant: true,
            violations: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
            required_actions: Vec::new(),
        };

        // Check each applicable rule
        for rule in self.rules.values() {
            if !rule.enabled {
                continue;
            }

            if rule.operation_types.contains(&operation_type) && 
               rule.data_classifications.contains(&data_classification) {
                
                let violation = self.check_rule(user_id, &operation_type, &data_classification, rule, context).await;
                
                if let Some(violation) = violation {
                    result.compliant = false;
                    result.violations.push(violation);
                }
            }
        }

        // Add contextual warnings and recommendations
        self.add_contextual_guidance(&mut result, &operation_type, &data_classification, context).await;

        // Log violations
        for violation in &result.violations {
            self.violations.push(violation.clone());
            tracing::warn!(
                "Compliance violation detected: {} - {} (Severity: {:?})",
                violation.rule_id,
                violation.description,
                violation.severity
            );
        }

        result
    }

    /// Check a specific compliance rule
    async fn check_rule(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        match rule.rule_id.as_str() {
            "ACP001" => self.check_attorney_client_privilege(user_id, operation_type, data_classification, rule, context).await,
            "UPL001" => self.check_unauthorized_practice(user_id, operation_type, data_classification, rule, context).await,
            "VRC001" => self.check_voice_recording_consent(user_id, operation_type, data_classification, rule, context).await,
            "DIP001" => self.check_document_integrity(user_id, operation_type, data_classification, rule, context).await,
            "HIPAA001" => self.check_hipaa_compliance(user_id, operation_type, data_classification, rule, context).await,
            "PII001" => self.check_pii_protection(user_id, operation_type, data_classification, rule, context).await,
            "WPP001" => self.check_work_product_protection(user_id, operation_type, data_classification, rule, context).await,
            _ => None,
        }
    }

    /// Check attorney-client privilege protection
    async fn check_attorney_client_privilege(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(data_classification, DataClassification::AttorneyClientPrivileged) {
            // Check if attorney supervision is documented
            let attorney_supervised = context
                .get("attorney_supervised")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !attorney_supervised {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Attorney-client privileged data processed without attorney supervision".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check unauthorized practice of law
    async fn check_unauthorized_practice(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(operation_type, LegalOperationType::AILegalAdvice) {
            let attorney_supervised = context
                .get("attorney_supervised")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let disclaimers_shown = context
                .get("disclaimers_shown")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !attorney_supervised || !disclaimers_shown {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "AI legal advice provided without proper supervision or disclaimers".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check voice recording consent
    async fn check_voice_recording_consent(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(operation_type, LegalOperationType::VoiceRecording) {
            let consent_given = context
                .get("consent_given")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !consent_given {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Voice recording initiated without proper consent".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check document integrity protection
    async fn check_document_integrity(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(operation_type, LegalOperationType::DocumentModification) {
            let chain_of_custody = context
                .get("chain_of_custody_maintained")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let version_control = context
                .get("version_control_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !chain_of_custody || !version_control {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Document modification without proper integrity controls".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check HIPAA compliance
    async fn check_hipaa_compliance(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(data_classification, DataClassification::HealthInformation) {
            let hipaa_authorized = context
                .get("hipaa_authorized")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !hipaa_authorized {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Health information processed without HIPAA authorization".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check PII protection
    async fn check_pii_protection(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(data_classification, DataClassification::PersonalIdentifiableInformation) {
            let pii_safeguards = context
                .get("pii_safeguards_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !pii_safeguards {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Personal information processed without adequate safeguards".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Check work product protection
    async fn check_work_product_protection(
        &self,
        user_id: &str,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        rule: &ComplianceRule,
        context: &HashMap<String, serde_json::Value>,
    ) -> Option<ComplianceViolation> {
        if matches!(data_classification, DataClassification::WorkProduct) {
            let work_product_protected = context
                .get("work_product_protected")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if !work_product_protected {
                return Some(ComplianceViolation {
                    violation_id: uuid::Uuid::new_v4().to_string(),
                    rule_id: rule.rule_id.clone(),
                    user_id: user_id.to_string(),
                    operation_type: operation_type.clone(),
                    data_classification: data_classification.clone(),
                    severity: rule.severity.clone(),
                    description: "Attorney work product processed without proper protection".to_string(),
                    timestamp: Utc::now(),
                    resolved: false,
                    resolution_notes: None,
                });
            }
        }
        None
    }

    /// Add contextual guidance based on operation and data
    async fn add_contextual_guidance(
        &self,
        result: &mut ComplianceCheckResult,
        operation_type: &LegalOperationType,
        data_classification: &DataClassification,
        _context: &HashMap<String, serde_json::Value>,
    ) {
        // Add warnings
        match operation_type {
            LegalOperationType::AILegalAdvice => {
                result.warnings.push("AI-generated legal content must be reviewed by a licensed attorney".to_string());
                result.required_actions.push("Obtain attorney review before using AI legal advice".to_string());
            }
            LegalOperationType::VoiceRecording => {
                result.warnings.push("Voice recordings may be subject to legal discovery".to_string());
                result.required_actions.push("Ensure all parties consent to recording".to_string());
            }
            LegalOperationType::DocumentModification => {
                result.warnings.push("Document modifications may affect legal proceedings".to_string());
                result.required_actions.push("Maintain detailed audit trail of all changes".to_string());
            }
            _ => {}
        }

        // Add data-specific warnings
        match data_classification {
            DataClassification::AttorneyClientPrivileged => {
                result.warnings.push("Privileged communications require special protection".to_string());
                result.recommendations.push("Implement encryption and access controls".to_string());
            }
            DataClassification::HealthInformation => {
                result.warnings.push("Health information subject to HIPAA requirements".to_string());
                result.required_actions.push("Verify HIPAA compliance before processing".to_string());
            }
            DataClassification::PersonalIdentifiableInformation => {
                result.recommendations.push("Consider data minimization principles".to_string());
                result.recommendations.push("Implement privacy-by-design practices".to_string());
            }
            _ => {}
        }

        // General recommendations
        result.recommendations.push("Regular compliance training for all users".to_string());
        result.recommendations.push("Periodic review of compliance policies".to_string());
    }

    /// Get all violations for a user
    pub async fn get_user_violations(&self, user_id: &str) -> Vec<&ComplianceViolation> {
        self.violations
            .iter()
            .filter(|v| v.user_id == user_id)
            .collect()
    }

    /// Get unresolved violations
    pub async fn get_unresolved_violations(&self) -> Vec<&ComplianceViolation> {
        self.violations
            .iter()
            .filter(|v| !v.resolved)
            .collect()
    }

    /// Resolve a violation
    pub async fn resolve_violation(&mut self, violation_id: &str, resolution_notes: String) -> Result<(), String> {
        if let Some(violation) = self.violations.iter_mut().find(|v| v.violation_id == violation_id) {
            violation.resolved = true;
            violation.resolution_notes = Some(resolution_notes);
            tracing::info!("Compliance violation {} resolved", violation_id);
            Ok(())
        } else {
            Err("Violation not found".to_string())
        }
    }

    /// Generate compliance report
    pub async fn generate_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> String {
        let mut report = String::from("=== COMPLIANCE VIOLATIONS REPORT ===\n\n");
        report.push_str(&format!("Period: {} to {}\n", start_date.format("%Y-%m-%d"), end_date.format("%Y-%m-%d")));
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        let period_violations: Vec<&ComplianceViolation> = self.violations
            .iter()
            .filter(|v| v.timestamp >= start_date && v.timestamp <= end_date)
            .collect();

        report.push_str(&format!("Total Violations: {}\n", period_violations.len()));

        // Count by severity
        let critical_count = period_violations.iter().filter(|v| matches!(v.severity, RuleSeverity::Critical)).count();
        let high_count = period_violations.iter().filter(|v| matches!(v.severity, RuleSeverity::High)).count();
        let medium_count = period_violations.iter().filter(|v| matches!(v.severity, RuleSeverity::Medium)).count();

        report.push_str(&format!("Critical: {}, High: {}, Medium: {}\n\n", critical_count, high_count, medium_count));

        // List violations
        report.push_str("VIOLATIONS:\n");
        for violation in period_violations {
            report.push_str(&format!(
                "- {} ({}): {} - {} [{}]\n",
                violation.violation_id,
                violation.timestamp.format("%Y-%m-%d %H:%M"),
                violation.rule_id,
                violation.description,
                if violation.resolved { "Resolved" } else { "Open" }
            ));
        }

        report
    }
}
