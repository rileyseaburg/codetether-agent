//! Application token-count projection for context status output.
use crate::tui::app::state::AppState;
/// Describe current token usage and the most recent context events.
///
/// # Arguments
/// * `state` - Current token estimates and recorded context-health events.
///
/// # Returns
/// A four-line context-health summary.
///
/// # Examples
/// ```no_run
/// # fn example(state: &codetether_agent::tui::app::state::AppState) {
/// let _summary = codetether_agent::tui::app::context_status::render(state);
/// # }
/// ```
pub fn render(state: &AppState) -> String {
    codetether_context_health::render(
        &state.context_health,
        state.context_health.used_tokens,
        state.context_health.token_budget,
    )
}
