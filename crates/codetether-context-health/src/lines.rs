//! Context usage and recent-event label construction.
use crate::ContextHealthState;

pub(super) fn usage_line(used: Option<usize>, budget: Option<usize>) -> String {
    match (used, budget) {
        (Some(used), Some(budget)) if budget > 0 => {
            format!(
                "Context: {used}/{budget} tokens ({:.1}%)",
                used as f64 * 100.0 / budget as f64
            )
        }
        (Some(used), _) => format!("Context: {used} tokens (budget unknown)"),
        _ => "Context: no estimate yet".to_string(),
    }
}

pub(super) fn compaction_line(state: &ContextHealthState) -> String {
    format!(
        "Last compaction: {}",
        state.last_compaction.as_deref().unwrap_or("none")
    )
}

pub(super) fn rlm_line(state: &ContextHealthState) -> String {
    format!("Last RLM: {}", state.last_rlm.as_deref().unwrap_or("none"))
}

pub(super) fn truncation_line(state: &ContextHealthState) -> String {
    format!(
        "Last truncation: {}",
        state.last_truncation.as_deref().unwrap_or("none")
    )
}
