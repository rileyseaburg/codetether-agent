//! Shared forage run and selection summaries.
use crate::ForageOpportunity;
use serde::Serialize;

/// Compact details for one opportunity selected in a cycle.
#[derive(Debug, Clone, Serialize)]
pub struct ForageSelectionSummary {
    pub okr_title: String,
    pub key_result_title: String,
    pub score: f64,
    pub progress: f64,
    pub remaining: f64,
    pub moonshot_alignment: f64,
}

/// Completed-cycle totals and selections for CLI or UI presentation.
///
/// # Examples
/// ```
/// use codetether_forage_core::ForageRunSummary;
/// let summary = ForageRunSummary { cycles_completed: 0, execute_requested: false, execution_engine: "run".into(), total_selected: 0, total_executed: 0, total_execution_failures: 0, last_cycle_selected: vec![] };
/// assert!(summary.render_text().contains("0 cycle(s)"));
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ForageRunSummary {
    pub cycles_completed: usize,
    pub execute_requested: bool,
    pub execution_engine: String,
    pub total_selected: usize,
    pub total_executed: usize,
    pub total_execution_failures: usize,
    pub last_cycle_selected: Vec<ForageSelectionSummary>,
}
impl ForageRunSummary {
    /// Project complete opportunity records into cycle-summary records.
    pub fn summarize_selection(selected: &[ForageOpportunity]) -> Vec<ForageSelectionSummary> {
        selected
            .iter()
            .map(|item| ForageSelectionSummary {
                okr_title: item.okr_title.clone(),
                key_result_title: item.key_result_title.clone(),
                score: item.score,
                progress: item.progress,
                remaining: item.remaining,
                moonshot_alignment: item.moonshot_alignment,
            })
            .collect()
    }
}
