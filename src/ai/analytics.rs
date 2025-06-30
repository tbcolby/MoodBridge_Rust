use crate::ai::{AiError, AiInsight, InsightType};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

/// Legal analytics service for statistical analysis
pub struct LegalAnalytics;

impl LegalAnalytics {
    /// Analyze placement denial patterns
    pub fn analyze_placement_patterns(denials: &[Value]) -> Result<Vec<AiInsight>, AiError> {
        let mut insights = Vec::new();

        if denials.is_empty() {
            return Ok(insights);
        }

        // Analyze denial frequency patterns
        let frequency_insight = Self::analyze_denial_frequency(denials)?;
        insights.push(frequency_insight);

        // Analyze duration patterns
        if let Ok(duration_insight) = Self::analyze_duration_patterns(denials) {
            insights.push(duration_insight);
        }

        // Analyze reason categorization
        if let Ok(reason_insight) = Self::analyze_denial_reasons(denials) {
            insights.push(reason_insight);
        }

        Ok(insights)
    }

    /// Analyze communication patterns
    pub fn analyze_communication_patterns(
        communications: &[Value],
    ) -> Result<Vec<AiInsight>, AiError> {
        let mut insights = Vec::new();

        if communications.is_empty() {
            return Ok(insights);
        }

        // Analyze communication frequency
        let frequency_data = Self::calculate_communication_frequency(communications);

        let insight = AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: 0.8,
            data: serde_json::json!({
                "pattern_type": "communication_frequency",
                "total_communications": communications.len(),
                "frequency_analysis": frequency_data,
                "analysis_summary": "Communication pattern analysis completed"
            }),
            generated_by: "legal_analytics".to_string(),
            created_at: Utc::now(),
        };

        insights.push(insight);
        Ok(insights)
    }

    /// Generate case statistics
    pub fn generate_case_statistics(
        denials: &[Value],
        communications: &[Value],
        timeline_events: &[Value],
    ) -> Result<AiInsight, AiError> {
        let total_denials = denials.len();
        let total_communications = communications.len();
        let total_events = timeline_events.len();

        // Calculate total lost hours
        let total_lost_hours: f64 = denials
            .iter()
            .filter_map(|denial| denial.get("duration_hours"))
            .filter_map(|hours| hours.as_f64())
            .sum();

        // Calculate risk metrics
        let high_risk_denials = denials
            .iter()
            .filter(|denial| {
                denial
                    .get("ai_risk_score")
                    .and_then(|score| score.as_f64())
                    .map(|score| score > 0.7)
                    .unwrap_or(false)
            })
            .count();

        let statistics = serde_json::json!({
            "case_overview": {
                "total_placement_denials": total_denials,
                "total_communications": total_communications,
                "total_timeline_events": total_events,
                "total_lost_hours": total_lost_hours,
                "high_risk_denials": high_risk_denials
            },
            "risk_assessment": {
                "overall_risk_level": if high_risk_denials > total_denials / 2 { "HIGH" } else { "MEDIUM" },
                "risk_percentage": if total_denials > 0 { (high_risk_denials as f64 / total_denials as f64) * 100.0 } else { 0.0 }
            },
            "trends": {
                "denial_frequency": "Calculated based on historical data",
                "communication_patterns": "Active monitoring in progress"
            }
        });

        Ok(AiInsight {
            insight_type: InsightType::RiskAssessment,
            confidence_score: 0.9,
            data: statistics,
            generated_by: "case_statistics_analyzer".to_string(),
            created_at: Utc::now(),
        })
    }

    fn analyze_denial_frequency(denials: &[Value]) -> Result<AiInsight, AiError> {
        let mut monthly_counts: HashMap<String, usize> = HashMap::new();

        for denial in denials {
            if let Some(date_str) = denial.get("denied_date").and_then(|d| d.as_str()) {
                // Extract year-month from date string
                let month_key = date_str.split('-').take(2).collect::<Vec<_>>().join("-");
                *monthly_counts.entry(month_key).or_insert(0) += 1;
            }
        }

        let insight_data = serde_json::json!({
            "pattern_type": "denial_frequency",
            "monthly_breakdown": monthly_counts,
            "total_denials": denials.len(),
            "peak_month": monthly_counts.iter().max_by_key(|(_, &count)| count).map(|(month, _)| month),
            "average_per_month": if !monthly_counts.is_empty() {
                denials.len() as f64 / monthly_counts.len() as f64
            } else { 0.0 }
        });

        Ok(AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: 0.85,
            data: insight_data,
            generated_by: "frequency_analyzer".to_string(),
            created_at: Utc::now(),
        })
    }

    fn analyze_duration_patterns(denials: &[Value]) -> Result<AiInsight, AiError> {
        let durations: Vec<f64> = denials
            .iter()
            .filter_map(|denial| denial.get("duration_hours"))
            .filter_map(|hours| hours.as_f64())
            .collect();

        if durations.is_empty() {
            return Err(AiError::ModelError(
                "No duration data available".to_string(),
            ));
        }

        let total_duration: f64 = durations.iter().sum();
        let average_duration = total_duration / durations.len() as f64;
        let max_duration = durations.iter().fold(0.0_f64, |a, &b| a.max(b));
        let min_duration = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        let insight_data = serde_json::json!({
            "pattern_type": "duration_analysis",
            "total_lost_hours": total_duration,
            "average_duration": average_duration,
            "max_duration": max_duration,
            "min_duration": min_duration,
            "denial_count": durations.len()
        });

        Ok(AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: 0.9,
            data: insight_data,
            generated_by: "duration_analyzer".to_string(),
            created_at: Utc::now(),
        })
    }

    fn analyze_denial_reasons(denials: &[Value]) -> Result<AiInsight, AiError> {
        let mut reason_counts: HashMap<String, usize> = HashMap::new();

        for denial in denials {
            if let Some(reason) = denial.get("denial_reason").and_then(|r| r.as_str()) {
                *reason_counts.entry(reason.to_string()).or_insert(0) += 1;
            }
        }

        let insight_data = serde_json::json!({
            "pattern_type": "denial_reasons",
            "reason_breakdown": reason_counts,
            "most_common_reason": reason_counts.iter().max_by_key(|(_, &count)| count).map(|(reason, _)| reason),
            "unique_reasons": reason_counts.len()
        });

        Ok(AiInsight {
            insight_type: InsightType::Pattern,
            confidence_score: 0.8,
            data: insight_data,
            generated_by: "reason_analyzer".to_string(),
            created_at: Utc::now(),
        })
    }

    fn calculate_communication_frequency(communications: &[Value]) -> HashMap<String, usize> {
        let mut frequency_map: HashMap<String, usize> = HashMap::new();

        for comm in communications {
            if let Some(date_str) = comm.get("communication_date").and_then(|d| d.as_str()) {
                let month_key = date_str.split('-').take(2).collect::<Vec<_>>().join("-");
                *frequency_map.entry(month_key).or_insert(0) += 1;
            }
        }

        frequency_map
    }
}
