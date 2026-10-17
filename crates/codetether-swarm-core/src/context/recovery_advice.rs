use super::exhaustion_report::TokenExhaustionReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resume and scope suggestions derived from an exhausted task.
///
/// # Examples
/// ```
/// let advice = codetether_swarm_core::context::RecoveryAdvice { can_resume: false, suggested_splits: vec![], completed_summary: String::new() };
/// assert!(!advice.can_resume);
/// ```
pub struct RecoveryAdvice {
    pub can_resume: bool,
    pub suggested_splits: Vec<String>,
    pub completed_summary: String,
}

impl RecoveryAdvice {
    /// Derive recovery suggestions from progress and changed files in a report.
    pub fn from_report(report: &TokenExhaustionReport) -> Self {
        let can_resume = report.steps_completed > 0;
        let mut suggested_splits = Vec::new();
        if !report.changed_files.is_empty() {
            suggested_splits.push(format!(
                "Resume editing: {}",
                report.changed_files.join(", ")
            ));
        }
        if report.steps_completed > 5 {
            suggested_splits.push(format!(
                "Continue from step {} with fresh context",
                report.steps_completed
            ));
        }
        if suggested_splits.is_empty() {
            suggested_splits.push("Retry with reduced scope".into());
        }
        let completed_summary = if report.steps_completed == 0 {
            "No steps completed before exhaustion.".into()
        } else {
            format!(
                "Completed {} steps, changed {} file(s).",
                report.steps_completed,
                report.changed_files.len()
            )
        };
        Self {
            can_resume,
            suggested_splits,
            completed_summary,
        }
    }
}
