use crate::ai::{AiError, AiInsight, InsightType, PatternConfig};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

/// Legal pattern detection service
pub struct PatternDetector {
    patterns: Vec<PatternConfig>,
}

impl PatternDetector {
    pub fn new() -> Self {
        Self {
            patterns: Self::initialize_default_patterns(),
        }
    }

    /// Add a new pattern to the detector
    pub fn add_pattern(&mut self, pattern: PatternConfig) {
        self.patterns.push(pattern);
    }

    /// Detect patterns in placement denials
    pub fn detect_placement_denial_patterns(
        &self,
        denials: &[Value],
    ) -> Result<Vec<AiInsight>, AiError> {
        let mut insights = Vec::new();

        for pattern in &self.patterns {
            if pattern.pattern_type == "violation" && pattern.active {
                if let Ok(insight) = self.apply_pattern_to_denials(pattern, denials) {
                    insights.push(insight);
                }
            }
        }

        Ok(insights)
    }

    /// Detect communication patterns
    pub fn detect_communication_patterns(
        &self,
        communications: &[Value],
    ) -> Result<Vec<AiInsight>, AiError> {
        let mut insights = Vec::new();

        for pattern in &self.patterns {
            if pattern.pattern_type == "communication" && pattern.active {
                if let Ok(insight) = self.apply_pattern_to_communications(pattern, communications) {
                    insights.push(insight);
                }
            }
        }

        Ok(insights)
    }

    /// Detect timeline correlation patterns
    pub fn detect_timeline_patterns(&self, events: &[Value]) -> Result<Vec<AiInsight>, AiError> {
        let mut insights = Vec::new();

        for pattern in &self.patterns {
            if pattern.pattern_type == "timeline" && pattern.active {
                if let Ok(insight) = self.apply_pattern_to_timeline(pattern, events) {
                    insights.push(insight);
                }
            }
        }

        Ok(insights)
    }

    fn apply_pattern_to_denials(
        &self,
        pattern: &PatternConfig,
        denials: &[Value],
    ) -> Result<AiInsight, AiError> {
        match pattern.pattern_name.as_str() {
            "Recurring Denial Pattern" => self.detect_recurring_denials(denials),
            _ => Err(AiError::ModelError("Unknown denial pattern".to_string())),
        }
    }

    fn apply_pattern_to_communications(
        &self,
        pattern: &PatternConfig,
        communications: &[Value],
    ) -> Result<AiInsight, AiError> {
        match pattern.pattern_name.as_str() {
            "Communication Gap" => self.detect_communication_gaps(communications),
            _ => Err(AiError::ModelError(
                "Unknown communication pattern".to_string(),
            )),
        }
    }

    fn apply_pattern_to_timeline(
        &self,
        pattern: &PatternConfig,
        events: &[Value],
    ) -> Result<AiInsight, AiError> {
        match pattern.pattern_name.as_str() {
            "Evidence Correlation" => self.detect_evidence_correlations(events),
            _ => Err(AiError::ModelError("Unknown timeline pattern".to_string())),
        }
    }

    /// Detect recurring denial patterns
    fn detect_recurring_denials(&self, denials: &[Value]) -> Result<AiInsight, AiError> {
        if denials.len() < 3 {
            return Err(AiError::ModelError(
                "Insufficient data for pattern detection".to_string(),
            ));
        }

        // Group denials by date proximity
        let mut date_clusters: Vec<Vec<&Value>> = Vec::new();
        let mut processed_indices = std::collections::HashSet::new();

        for (i, denial) in denials.iter().enumerate() {
            if processed_indices.contains(&i) {
                continue;
            }

            let mut cluster = vec![denial];
            processed_indices.insert(i);

            if let Some(date_str) = denial.get("denied_date").and_then(|d| d.as_str()) {
                // Find other denials within 30 days
                for (j, other_denial) in denials.iter().enumerate() {
                    if i != j && !processed_indices.contains(&j) {
                        if let Some(other_date_str) =
                            other_denial.get("denied_date").and_then(|d| d.as_str())
                        {
                            // Simple date comparison (would need proper date parsing in production)
                            if Self::dates_within_days(date_str, other_date_str, 30) {
                                cluster.push(other_denial);
                                processed_indices.insert(j);
                            }
                        }
                    }
                }
            }

            if cluster.len() >= 3 {
                date_clusters.push(cluster);
            }
        }

        let pattern_detected = !date_clusters.is_empty();
        let confidence = if pattern_detected { 0.8 } else { 0.2 };

        let insight_data = serde_json::json!({
            "pattern_name": "Recurring Denial Pattern",
            "pattern_detected": pattern_detected,
            "clusters_found": date_clusters.len(),
            "largest_cluster_size": date_clusters.iter().map(|c| c.len()).max().unwrap_or(0),
            "severity": if date_clusters.len() > 1 { "HIGH" } else { "MEDIUM" },
            "recommendation": if pattern_detected {
                "Multiple recurring denial patterns detected. Consider legal review for potential procedural violations."
            } else {
                "No significant recurring patterns detected."
            }
        });

        Ok(AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: confidence,
            data: insight_data,
            generated_by: "recurring_denial_detector".to_string(),
            created_at: Utc::now(),
        })
    }

    /// Detect communication gaps
    fn detect_communication_gaps(&self, communications: &[Value]) -> Result<AiInsight, AiError> {
        if communications.len() < 2 {
            return Err(AiError::ModelError(
                "Insufficient communication data".to_string(),
            ));
        }

        let mut gaps_detected = Vec::new();
        let mut sorted_comms: Vec<&Value> = communications.iter().collect();

        // Sort by date (simplified - would need proper date parsing)
        sorted_comms.sort_by(|a, b| {
            let date_a = a
                .get("communication_date")
                .and_then(|d| d.as_str())
                .unwrap_or("");
            let date_b = b
                .get("communication_date")
                .and_then(|d| d.as_str())
                .unwrap_or("");
            date_a.cmp(date_b)
        });

        // Check for gaps > 7 days between communications
        for window in sorted_comms.windows(2) {
            if let (Some(date1), Some(date2)) = (
                window[0].get("communication_date").and_then(|d| d.as_str()),
                window[1].get("communication_date").and_then(|d| d.as_str()),
            ) {
                if Self::days_between_dates(date1, date2) > 7 {
                    gaps_detected.push(serde_json::json!({
                        "gap_start": date1,
                        "gap_end": date2,
                        "estimated_days": Self::days_between_dates(date1, date2)
                    }));
                }
            }
        }

        let pattern_detected = !gaps_detected.is_empty();
        let confidence = if pattern_detected { 0.75 } else { 0.3 };

        let insight_data = serde_json::json!({
            "pattern_name": "Communication Gap",
            "pattern_detected": pattern_detected,
            "gaps_found": gaps_detected.len(),
            "gaps_details": gaps_detected,
            "severity": if gaps_detected.len() > 3 { "HIGH" } else { "MEDIUM" },
            "recommendation": if pattern_detected {
                "Significant communication gaps detected. Consider establishing regular communication protocols."
            } else {
                "Communication frequency appears adequate."
            }
        });

        Ok(AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: confidence,
            data: insight_data,
            generated_by: "communication_gap_detector".to_string(),
            created_at: Utc::now(),
        })
    }

    /// Detect evidence correlations in timeline
    fn detect_evidence_correlations(&self, events: &[Value]) -> Result<AiInsight, AiError> {
        if events.is_empty() {
            return Err(AiError::ModelError(
                "No timeline events to analyze".to_string(),
            ));
        }

        let mut correlations = Vec::new();
        let evidence_events: Vec<&Value> = events
            .iter()
            .filter(|event| {
                event
                    .get("event_type")
                    .and_then(|t| t.as_str())
                    .map(|t| t.contains("evidence") || t.contains("document"))
                    .unwrap_or(false)
            })
            .collect();

        let denial_events: Vec<&Value> = events
            .iter()
            .filter(|event| {
                event
                    .get("event_type")
                    .and_then(|t| t.as_str())
                    .map(|t| t.contains("denial") || t.contains("placement"))
                    .unwrap_or(false)
            })
            .collect();

        // Find evidence events that occur close to denial events
        for evidence in &evidence_events {
            for denial in &denial_events {
                if let (Some(ev_date), Some(den_date)) = (
                    evidence.get("event_date").and_then(|d| d.as_str()),
                    denial.get("event_date").and_then(|d| d.as_str()),
                ) {
                    if Self::days_between_dates(ev_date, den_date).abs() <= 7 {
                        correlations.push(serde_json::json!({
                            "evidence_event": evidence.get("event_title").and_then(|t| t.as_str()).unwrap_or("Unknown"),
                            "denial_event": denial.get("event_title").and_then(|t| t.as_str()).unwrap_or("Unknown"),
                            "evidence_date": ev_date,
                            "denial_date": den_date,
                            "correlation_strength": "STRONG"
                        }));
                    }
                }
            }
        }

        let pattern_detected = !correlations.is_empty();
        let confidence = if pattern_detected { 0.7 } else { 0.4 };

        let insight_data = serde_json::json!({
            "pattern_name": "Evidence Correlation",
            "pattern_detected": pattern_detected,
            "correlations_found": correlations.len(),
            "correlations": correlations,
            "evidence_events_count": evidence_events.len(),
            "denial_events_count": denial_events.len(),
            "recommendation": if pattern_detected {
                "Strong correlations found between evidence and denial events. Review for causal relationships."
            } else {
                "No significant evidence-denial correlations detected."
            }
        });

        Ok(AiInsight {
            insight_type: InsightType::TimelineCorrelation,
            confidence_score: confidence,
            data: insight_data,
            generated_by: "evidence_correlation_detector".to_string(),
            created_at: Utc::now(),
        })
    }

    /// Initialize default legal patterns
    fn initialize_default_patterns() -> Vec<PatternConfig> {
        vec![
            PatternConfig {
                pattern_name: "Recurring Denial Pattern".to_string(),
                pattern_type: "violation".to_string(),
                detection_criteria: {
                    let mut criteria = HashMap::new();
                    criteria.insert("min_denials".to_string(), serde_json::json!(3));
                    criteria.insert("time_window_days".to_string(), serde_json::json!(30));
                    criteria
                },
                severity_weight: 0.8,
                active: true,
            },
            PatternConfig {
                pattern_name: "Communication Gap".to_string(),
                pattern_type: "communication".to_string(),
                detection_criteria: {
                    let mut criteria = HashMap::new();
                    criteria.insert("max_gap_days".to_string(), serde_json::json!(7));
                    criteria.insert(
                        "critical_periods".to_string(),
                        serde_json::json!(["pre_placement", "post_denial"]),
                    );
                    criteria
                },
                severity_weight: 0.6,
                active: true,
            },
            PatternConfig {
                pattern_name: "Evidence Correlation".to_string(),
                pattern_type: "timeline".to_string(),
                detection_criteria: {
                    let mut criteria = HashMap::new();
                    criteria.insert(
                        "evidence_types".to_string(),
                        serde_json::json!(["document", "communication"]),
                    );
                    criteria.insert("correlation_threshold".to_string(), serde_json::json!(0.8));
                    criteria
                },
                severity_weight: 0.7,
                active: true,
            },
        ]
    }

    // Helper functions (simplified - would need proper date parsing library in production)
    fn dates_within_days(date1: &str, date2: &str, days: i32) -> bool {
        Self::days_between_dates(date1, date2).abs() <= days
    }

    fn days_between_dates(date1: &str, date2: &str) -> i32 {
        // Simplified date comparison - would use chrono in production
        // For now, just compare string values lexicographically
        match date1.cmp(date2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}
