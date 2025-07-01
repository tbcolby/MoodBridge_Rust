/// Audit Logging Module for MoodBridge_Rust
/// 
/// This module provides comprehensive audit logging for all legal operations,
/// ensuring compliance with legal and regulatory requirements for tracking
/// and documentation of system activities.

use crate::legal::{AuditLogEntry, LegalOperationType, ComplianceStatus, DataClassification};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Audit log search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSearchCriteria {
    pub user_id: Option<String>,
    pub operation_type: Option<LegalOperationType>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub compliance_status: Option<ComplianceStatus>,
    pub data_classification: Option<DataClassification>,
    pub limit: Option<usize>,
}

/// Audit log statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    pub total_entries: usize,
    pub entries_by_operation: HashMap<String, usize>,
    pub entries_by_user: HashMap<String, usize>,
    pub entries_by_compliance_status: HashMap<String, usize>,
    pub attorney_review_required_count: usize,
    pub privileged_data_operations: usize,
    pub recent_activity_count: usize,
}

/// Audit log export format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    JSON,
    CSV,
    PDF,
    XML,
}

/// Audit logger for legal operations
#[derive(Debug, Clone)]
pub struct AuditLogger {
    log_entries: Vec<AuditLogEntry>,
    max_entries: usize,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            log_entries: Vec::new(),
            max_entries: 100_000, // Keep last 100,000 entries in memory
        }
    }

    /// Log a legal operation
    pub async fn log_operation(&mut self, entry: &AuditLogEntry) {
        self.log_entries.push(entry.clone());
        
        // Maintain maximum entries limit
        if self.log_entries.len() > self.max_entries {
            // Remove oldest entries but keep important ones
            self.log_entries.retain(|e| {
                // Always keep entries that required attorney review
                if e.attorney_review_required {
                    return true;
                }
                
                // Always keep entries with privileged data
                if matches!(e.data_processed, DataClassification::AttorneyClientPrivileged | DataClassification::WorkProduct) {
                    return true;
                }
                
                // Keep recent entries (last 90 days)
                let ninety_days_ago = Utc::now() - chrono::Duration::days(90);
                e.timestamp > ninety_days_ago
            });
        }

        tracing::info!(
            "Audit log entry recorded: user={}, operation={:?}, compliance={:?}",
            entry.user_id,
            entry.operation_type,
            entry.compliance_status
        );
    }

    /// Search audit log entries
    pub async fn search_entries(&self, criteria: &AuditSearchCriteria) -> Vec<&AuditLogEntry> {
        let mut results: Vec<&AuditLogEntry> = self.log_entries
            .iter()
            .filter(|entry| {
                // Filter by user ID
                if let Some(ref user_id) = criteria.user_id {
                    if entry.user_id != *user_id {
                        return false;
                    }
                }

                // Filter by operation type
                if let Some(ref operation_type) = criteria.operation_type {
                    if entry.operation_type != *operation_type {
                        return false;
                    }
                }

                // Filter by date range
                if let Some(start_date) = criteria.start_date {
                    if entry.timestamp < start_date {
                        return false;
                    }
                }

                if let Some(end_date) = criteria.end_date {
                    if entry.timestamp > end_date {
                        return false;
                    }
                }

                // Filter by compliance status
                if let Some(ref compliance_status) = criteria.compliance_status {
                    if entry.compliance_status != *compliance_status {
                        return false;
                    }
                }

                // Filter by data classification
                if let Some(ref data_classification) = criteria.data_classification {
                    if entry.data_processed != *data_classification {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Sort by timestamp (newest first)
        results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = criteria.limit {
            results.truncate(limit);
        }

        results
    }

    /// Get audit statistics
    pub async fn get_statistics(&self) -> AuditStatistics {
        let mut stats = AuditStatistics {
            total_entries: self.log_entries.len(),
            entries_by_operation: HashMap::new(),
            entries_by_user: HashMap::new(),
            entries_by_compliance_status: HashMap::new(),
            attorney_review_required_count: 0,
            privileged_data_operations: 0,
            recent_activity_count: 0,
        };

        let recent_threshold = Utc::now() - chrono::Duration::hours(24);

        for entry in &self.log_entries {
            // Count by operation type
            let operation_key = format!("{:?}", entry.operation_type);
            *stats.entries_by_operation.entry(operation_key).or_insert(0) += 1;

            // Count by user
            *stats.entries_by_user.entry(entry.user_id.clone()).or_insert(0) += 1;

            // Count by compliance status
            let compliance_key = format!("{:?}", entry.compliance_status);
            *stats.entries_by_compliance_status.entry(compliance_key).or_insert(0) += 1;

            // Count attorney review required
            if entry.attorney_review_required {
                stats.attorney_review_required_count += 1;
            }

            // Count privileged data operations
            if matches!(entry.data_processed, 
                DataClassification::AttorneyClientPrivileged | 
                DataClassification::WorkProduct
            ) {
                stats.privileged_data_operations += 1;
            }

            // Count recent activity
            if entry.timestamp > recent_threshold {
                stats.recent_activity_count += 1;
            }
        }

        stats
    }

    /// Export audit log
    pub async fn export_audit_log(&self, criteria: &AuditSearchCriteria, format: ExportFormat) -> Result<String, String> {
        let entries = self.search_entries(criteria).await;

        match format {
            ExportFormat::JSON => {
                serde_json::to_string_pretty(&entries)
                    .map_err(|e| format!("JSON export failed: {}", e))
            }
            ExportFormat::CSV => {
                self.export_to_csv(&entries).await
            }
            ExportFormat::PDF => {
                self.export_to_pdf(&entries).await
            }
            ExportFormat::XML => {
                self.export_to_xml(&entries).await
            }
        }
    }

    /// Export to CSV format
    async fn export_to_csv(&self, entries: &[&AuditLogEntry]) -> Result<String, String> {
        let mut csv = String::from("Timestamp,User ID,Operation Type,Compliance Status,Attorney Review Required,Data Classification,Entry ID\n");

        for entry in entries {
            csv.push_str(&format!(
                "{},{},{:?},{:?},{},{:?},{}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                entry.user_id,
                entry.operation_type,
                entry.compliance_status,
                entry.attorney_review_required,
                entry.data_processed,
                entry.entry_id
            ));
        }

        Ok(csv)
    }

    /// Export to PDF format (simplified for demo)
    async fn export_to_pdf(&self, entries: &[&AuditLogEntry]) -> Result<String, String> {
        let mut pdf_content = String::from("AUDIT LOG REPORT\n");
        pdf_content.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        for entry in entries {
            pdf_content.push_str(&format!(
                "Entry ID: {}\n",
                entry.entry_id
            ));
            pdf_content.push_str(&format!(
                "Timestamp: {}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
            pdf_content.push_str(&format!(
                "User: {}\n",
                entry.user_id
            ));
            pdf_content.push_str(&format!(
                "Operation: {:?}\n",
                entry.operation_type
            ));
            pdf_content.push_str(&format!(
                "Compliance Status: {:?}\n",
                entry.compliance_status
            ));
            pdf_content.push_str(&format!(
                "Attorney Review Required: {}\n",
                entry.attorney_review_required
            ));
            pdf_content.push_str(&format!(
                "Data Classification: {:?}\n",
                entry.data_processed
            ));
            pdf_content.push_str("---\n\n");
        }

        Ok(pdf_content)
    }

    /// Export to XML format
    async fn export_to_xml(&self, entries: &[&AuditLogEntry]) -> Result<String, String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<audit_log>\n");
        xml.push_str(&format!("  <generated_at>{}</generated_at>\n", Utc::now().to_rfc3339()));
        xml.push_str("  <entries>\n");

        for entry in entries {
            xml.push_str("    <entry>\n");
            xml.push_str(&format!("      <id>{}</id>\n", entry.entry_id));
            xml.push_str(&format!("      <timestamp>{}</timestamp>\n", entry.timestamp.to_rfc3339()));
            xml.push_str(&format!("      <user_id>{}</user_id>\n", entry.user_id));
            xml.push_str(&format!("      <operation_type>{:?}</operation_type>\n", entry.operation_type));
            xml.push_str(&format!("      <compliance_status>{:?}</compliance_status>\n", entry.compliance_status));
            xml.push_str(&format!("      <attorney_review_required>{}</attorney_review_required>\n", entry.attorney_review_required));
            xml.push_str(&format!("      <data_classification>{:?}</data_classification>\n", entry.data_processed));
            xml.push_str("    </entry>\n");
        }

        xml.push_str("  </entries>\n");
        xml.push_str("</audit_log>\n");

        Ok(xml)
    }

    /// Generate compliance report
    pub async fn generate_compliance_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> String {
        let criteria = AuditSearchCriteria {
            user_id: None,
            operation_type: None,
            start_date: Some(start_date),
            end_date: Some(end_date),
            compliance_status: None,
            data_classification: None,
            limit: None,
        };

        let entries = self.search_entries(&criteria).await;
        let stats = self.get_statistics().await;

        let mut report = String::from("=== LEGAL COMPLIANCE AUDIT REPORT ===\n\n");
        report.push_str(&format!("Report Period: {} to {}\n", 
            start_date.format("%Y-%m-%d"), 
            end_date.format("%Y-%m-%d")
        ));
        report.push_str(&format!("Generated: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Summary statistics
        report.push_str("SUMMARY STATISTICS:\n");
        report.push_str(&format!("  Total Operations: {}\n", entries.len()));
        report.push_str(&format!("  Attorney Review Required: {}\n", stats.attorney_review_required_count));
        report.push_str(&format!("  Privileged Data Operations: {}\n", stats.privileged_data_operations));
        report.push_str(&format!("  Recent Activity (24h): {}\n\n", stats.recent_activity_count));

        // Operations by type
        report.push_str("OPERATIONS BY TYPE:\n");
        for (operation, count) in &stats.entries_by_operation {
            report.push_str(&format!("  {}: {}\n", operation, count));
        }
        report.push_str("\n");

        // Compliance status breakdown
        report.push_str("COMPLIANCE STATUS BREAKDOWN:\n");
        for (status, count) in &stats.entries_by_compliance_status {
            report.push_str(&format!("  {}: {}\n", status, count));
        }
        report.push_str("\n");

        // Top users by activity
        report.push_str("TOP USERS BY ACTIVITY:\n");
        let mut user_activity: Vec<(&String, &usize)> = stats.entries_by_user.iter().collect();
        user_activity.sort_by(|a, b| b.1.cmp(a.1));
        for (user_id, count) in user_activity.iter().take(10) {
            report.push_str(&format!("  {}: {} operations\n", user_id, count));
        }
        report.push_str("\n");

        // Critical findings
        report.push_str("CRITICAL FINDINGS:\n");
        let mut critical_count = 0;

        // Check for operations without proper consent
        for entry in &entries {
            if matches!(entry.compliance_status, ComplianceStatus::RequiresConsent) {
                critical_count += 1;
            }
        }

        if critical_count > 0 {
            report.push_str(&format!("  ⚠️ {} operations performed without proper consent\n", critical_count));
        }

        // Check for privileged data exposure risk
        let privileged_without_review = entries.iter().filter(|e| {
            matches!(e.data_processed, DataClassification::AttorneyClientPrivileged) && 
            !e.attorney_review_required
        }).count();

        if privileged_without_review > 0 {
            report.push_str(&format!("  🚨 {} privileged data operations without attorney review\n", privileged_without_review));
        }

        if critical_count == 0 && privileged_without_review == 0 {
            report.push_str("  ✅ No critical compliance issues found\n");
        }

        report.push_str("\n");

        // Recommendations
        report.push_str("RECOMMENDATIONS:\n");
        if critical_count > 0 {
            report.push_str("  • Implement stronger consent verification procedures\n");
        }
        if privileged_without_review > 0 {
            report.push_str("  • Require attorney review for all privileged data operations\n");
        }
        if stats.attorney_review_required_count > stats.total_entries / 2 {
            report.push_str("  • Consider automation opportunities with proper safeguards\n");
        }
        report.push_str("  • Regular compliance training for all users\n");
        report.push_str("  • Periodic audit of user permissions and access\n");

        report
    }

    /// Archive old audit logs
    pub async fn archive_old_logs(&mut self, archive_before: DateTime<Utc>) -> usize {
        let initial_count = self.log_entries.len();
        
        self.log_entries.retain(|entry| {
            // Keep entries newer than archive date
            if entry.timestamp > archive_before {
                return true;
            }
            
            // Always keep privileged data operations regardless of age
            if matches!(entry.data_processed, 
                DataClassification::AttorneyClientPrivileged | 
                DataClassification::WorkProduct
            ) {
                return true;
            }
            
            // Always keep entries that required attorney review
            if entry.attorney_review_required {
                return true;
            }
            
            false
        });

        let archived_count = initial_count - self.log_entries.len();
        tracing::info!("Archived {} old audit log entries", archived_count);
        
        archived_count
    }

    /// Get entries requiring attorney review
    pub async fn get_pending_attorney_reviews(&self) -> Vec<&AuditLogEntry> {
        self.log_entries
            .iter()
            .filter(|entry| {
                entry.attorney_review_required && 
                matches!(entry.compliance_status, ComplianceStatus::RequiresAttorneyReview)
            })
            .collect()
    }

    /// Clear all audit logs (administrative function)
    pub async fn clear_all_logs(&mut self, cleared_by: &str) -> Result<usize, String> {
        let count = self.log_entries.len();
        self.log_entries.clear();
        
        tracing::warn!("All audit logs cleared by user: {}", cleared_by);
        
        Ok(count)
    }
}
