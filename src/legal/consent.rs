/// Consent Management Module for MoodBridge_Rust
/// 
/// This module handles user consent for legally sensitive operations,
/// ensuring proper documentation and compliance with legal requirements.

use crate::legal::{ConsentRecord, LegalOperationType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Consent status for a specific operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentStatus {
    pub consent_given: bool,
    pub timestamp: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub attorney_supervised: bool,
    pub requires_renewal: bool,
}

/// Consent manager for tracking and validating user consent
#[derive(Debug, Clone)]
pub struct ConsentManager {
    consent_records: HashMap<String, HashMap<String, ConsentRecord>>,
    consent_expiry_hours: u64,
}

impl ConsentManager {
    pub fn new() -> Self {
        Self {
            consent_records: HashMap::new(),
            consent_expiry_hours: 24, // Consent expires after 24 hours by default
        }
    }

    /// Record user consent for a specific operation
    pub async fn record_consent(&mut self, consent_record: ConsentRecord) -> Result<(), String> {
        let user_id = consent_record.user_id.clone();
        let operation_key = format!("{:?}", consent_record.operation_type);

        // Validate consent record
        if consent_record.consent_text.is_empty() {
            return Err("Consent text cannot be empty".to_string());
        }

        // Store the consent record
        self.consent_records
            .entry(user_id)
            .or_insert_with(HashMap::new)
            .insert(operation_key, consent_record);

        tracing::info!("Consent recorded for user and operation");
        Ok(())
    }

    /// Check if user has valid consent for an operation
    pub async fn check_consent(&self, user_id: &str, operation_type: &LegalOperationType) -> ConsentStatus {
        let operation_key = format!("{:?}", operation_type);
        
        if let Some(user_consents) = self.consent_records.get(user_id) {
            if let Some(consent_record) = user_consents.get(&operation_key) {
                // Check if consent has expired
                let expiry_time = consent_record.timestamp + Duration::hours(self.consent_expiry_hours as i64);
                let now = Utc::now();

                if now > expiry_time {
                    return ConsentStatus {
                        consent_given: false,
                        timestamp: Some(consent_record.timestamp),
                        expires_at: Some(expiry_time),
                        attorney_supervised: consent_record.attorney_supervised,
                        requires_renewal: true,
                    };
                }

                return ConsentStatus {
                    consent_given: consent_record.consent_given,
                    timestamp: Some(consent_record.timestamp),
                    expires_at: Some(expiry_time),
                    attorney_supervised: consent_record.attorney_supervised,
                    requires_renewal: false,
                };
            }
        }

        // No consent found
        ConsentStatus {
            consent_given: false,
            timestamp: None,
            expires_at: None,
            attorney_supervised: false,
            requires_renewal: false,
        }
    }

    /// Revoke consent for a specific operation
    pub async fn revoke_consent(&mut self, user_id: &str, operation_type: &LegalOperationType) -> Result<(), String> {
        let operation_key = format!("{:?}", operation_type);
        
        if let Some(user_consents) = self.consent_records.get_mut(user_id) {
            if let Some(consent_record) = user_consents.get_mut(&operation_key) {
                consent_record.consent_given = false;
                consent_record.timestamp = Utc::now();
                tracing::info!("Consent revoked for user and operation");
                return Ok(());
            }
        }

        Err("No consent record found to revoke".to_string())
    }

    /// Get all consent records for a user
    pub async fn get_user_consents(&self, user_id: &str) -> Vec<ConsentRecord> {
        if let Some(user_consents) = self.consent_records.get(user_id) {
            user_consents.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Check if consent is required for an operation type
    pub fn requires_consent(operation_type: &LegalOperationType) -> bool {
        match operation_type {
            LegalOperationType::AILegalAdvice => true,
            LegalOperationType::VoiceRecording => true,
            LegalOperationType::DocumentModification => true,
            LegalOperationType::ClientDataProcessing => true,
            LegalOperationType::TimelineAnalysis => true,
            LegalOperationType::DocumentIntelligence => true,
            LegalOperationType::PresentationGeneration => true,
            LegalOperationType::SemanticSearch => true,
            LegalOperationType::CollaborationMetrics => true,
        }
    }

    /// Check if attorney supervision is required for an operation
    pub fn requires_attorney_supervision(operation_type: &LegalOperationType) -> bool {
        match operation_type {
            LegalOperationType::AILegalAdvice => true,
            LegalOperationType::DocumentModification => true,
            LegalOperationType::ClientDataProcessing => true,
            LegalOperationType::TimelineAnalysis => true,
            _ => false,
        }
    }

    /// Get consent form HTML for an operation
    pub fn get_consent_form_html(&self, operation_type: &LegalOperationType, disclaimer_text: &str) -> String {
        let operation_name = match operation_type {
            LegalOperationType::AILegalAdvice => "AI Legal Assistance",
            LegalOperationType::VoiceRecording => "Voice Recording",
            LegalOperationType::DocumentModification => "Document Modification",
            LegalOperationType::ClientDataProcessing => "Client Data Processing",
            LegalOperationType::TimelineAnalysis => "Timeline Analysis",
            LegalOperationType::DocumentIntelligence => "Document Intelligence",
            LegalOperationType::PresentationGeneration => "Presentation Generation",
            LegalOperationType::SemanticSearch => "Semantic Search",
            LegalOperationType::CollaborationMetrics => "Collaboration Metrics",
        };

        let attorney_supervision_required = Self::requires_attorney_supervision(operation_type);

        format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Legal Consent Required - {}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background: #f5f5f5;
        }}
        .consent-container {{
            background: white;
            border-radius: 10px;
            padding: 30px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            border-left: 5px solid #dc3545;
        }}
        .warning-header {{
            background: #dc3545;
            color: white;
            padding: 15px;
            margin: -30px -30px 20px -30px;
            border-radius: 10px 10px 0 0;
            text-align: center;
        }}
        .disclaimer {{
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            border-radius: 5px;
            padding: 20px;
            margin: 20px 0;
            white-space: pre-line;
            font-size: 14px;
            line-height: 1.6;
            max-height: 300px;
            overflow-y: auto;
        }}
        .consent-form {{
            margin-top: 30px;
            padding-top: 20px;
            border-top: 2px solid #eee;
        }}
        .checkbox-group {{
            margin: 15px 0;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 5px;
        }}
        .checkbox-group input[type="checkbox"] {{
            margin-right: 10px;
            transform: scale(1.2);
        }}
        .checkbox-group label {{
            font-weight: 500;
            cursor: pointer;
        }}
        .attorney-supervision {{
            background: #e7f3ff;
            border: 2px solid #0056b3;
            border-radius: 5px;
            padding: 15px;
            margin: 20px 0;
        }}
        .buttons {{
            text-align: center;
            margin-top: 30px;
        }}
        .btn {{
            padding: 12px 30px;
            margin: 0 10px;
            border: none;
            border-radius: 5px;
            font-size: 16px;
            cursor: pointer;
            font-weight: 500;
        }}
        .btn-danger {{
            background: #dc3545;
            color: white;
        }}
        .btn-success {{
            background: #28a745;
            color: white;
        }}
        .btn:disabled {{
            background: #6c757d;
            cursor: not-allowed;
        }}
        .timestamp {{
            font-size: 12px;
            color: #666;
            text-align: center;
            margin-top: 20px;
        }}
    </style>
</head>
<body>
    <div class="consent-container">
        <div class="warning-header">
            <h2>⚖️ LEGAL CONSENT REQUIRED ⚖️</h2>
            <p>Operation: {}</p>
        </div>

        <h3>Legal Disclaimer and Warning</h3>
        <div class="disclaimer">{}</div>

        {}

        <div class="consent-form">
            <h3>Consent Requirements</h3>
            
            <div class="checkbox-group">
                <label>
                    <input type="checkbox" id="understand-disclaimer" required>
                    I have read and understand the legal disclaimer above
                </label>
            </div>

            <div class="checkbox-group">
                <label>
                    <input type="checkbox" id="accept-risks" required>
                    I understand the legal risks and limitations of this technology
                </label>
            </div>

            <div class="checkbox-group">
                <label>
                    <input type="checkbox" id="not-legal-advice" required>
                    I acknowledge this system does not provide legal advice
                </label>
            </div>

            <div class="checkbox-group">
                <label>
                    <input type="checkbox" id="attorney-consultation" required>
                    I agree to consult with a qualified attorney for legal matters
                </label>
            </div>

            <div class="checkbox-group">
                <label>
                    <input type="checkbox" id="accept-responsibility" required>
                    I accept full responsibility for the consequences of using this system
                </label>
            </div>

            <div class="buttons">
                <button type="button" class="btn btn-danger" onclick="window.history.back()">
                    Decline - Go Back
                </button>
                <button type="button" class="btn btn-success" id="consent-button" disabled onclick="submitConsent()">
                    I Consent - Proceed
                </button>
            </div>
        </div>

        <div class="timestamp">
            Consent recorded: <span id="timestamp"></span>
        </div>
    </div>

    <script>
        // Enable consent button only when all checkboxes are checked
        const checkboxes = document.querySelectorAll('input[type="checkbox"]');
        const consentButton = document.getElementById('consent-button');

        checkboxes.forEach(checkbox => {{
            checkbox.addEventListener('change', updateConsentButton);
        }});

        function updateConsentButton() {{
            const allChecked = Array.from(checkboxes).every(cb => cb.checked);
            consentButton.disabled = !allChecked;
        }}

        function submitConsent() {{
            if (Array.from(checkboxes).every(cb => cb.checked)) {{
                document.getElementById('timestamp').textContent = new Date().toISOString();
                
                // In a real implementation, this would POST to a consent endpoint
                const consentData = {{
                    operation_type: '{}',
                    consent_given: true,
                    timestamp: new Date().toISOString(),
                    ip_address: null, // Would be filled by server
                    attorney_supervised: {},
                    disclaimer_acknowledged: true
                }};

                // Show success message and redirect
                alert('Consent recorded successfully. You may now proceed with the operation.');
                
                // Redirect to the original operation
                window.location.href = '/api/consent/recorded?operation={}';
            }}
        }}
    </script>
</body>
</html>"#,
            operation_name,
            operation_name,
            disclaimer_text,
            if attorney_supervision_required {
                r#"<div class="attorney-supervision">
                    <h4>⚖️ ATTORNEY SUPERVISION REQUIRED</h4>
                    <p>This operation requires attorney supervision. Ensure that a qualified attorney is overseeing this process and will review all outputs before use.</p>
                </div>"#
            } else {
                ""
            },
            format!("{:?}", operation_type),
            attorney_supervision_required.to_string(),
            format!("{:?}", operation_type)
        )
    }

    /// Set custom consent expiry time
    pub fn set_consent_expiry_hours(&mut self, hours: u64) {
        self.consent_expiry_hours = hours;
    }

    /// Clean up expired consent records
    pub async fn cleanup_expired_consents(&mut self) {
        let now = Utc::now();
        let expiry_duration = Duration::hours(self.consent_expiry_hours as i64);

        for user_consents in self.consent_records.values_mut() {
            user_consents.retain(|_, consent| {
                consent.timestamp + expiry_duration > now
            });
        }
    }
}
