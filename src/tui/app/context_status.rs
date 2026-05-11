//! Render context/RLM status for slash commands.

use crate::tui::app::state::AppState;

pub fn render(state: &AppState) -> String {
    [
        usage_line(state),
        compaction_line(state),
        rlm_line(state),
        truncation_line(state),
    ]
    .join("\n")
}

fn usage_line(state: &AppState) -> String {
    match (state.context_used, state.context_budget) {
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

fn compaction_line(state: &AppState) -> String {
    format!(
        "Last compaction: {}",
        state
            .context_health
            .last_compaction
            .as_deref()
            .unwrap_or("none")
    )
}

fn rlm_line(state: &AppState) -> String {
    format!(
        "Last RLM: {}",
        state.context_health.last_rlm.as_deref().unwrap_or("none")
    )
}

fn truncation_line(state: &AppState) -> String {
    format!(
        "Last truncation: {}",
        state
            .context_health
            .last_truncation
            .as_deref()
            .unwrap_or("none")
    )
}
