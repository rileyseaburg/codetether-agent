//! Registered-agent projection for the shared status rail.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Draw registered worker names using their stable identity colors.
///
/// # Arguments
/// * `app` - Application values supplied to the shared renderer.
///
/// # Returns
/// Owned status display spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::agent_rail::agent_rail_spans(app);
/// # }
/// ```
pub fn agent_rail_spans(app: &App) -> Vec<Span<'static>> {
    codetether_chat_status::agent_rail_spans(
        app.state
            .worker
            .worker_bridge_registered_agents
            .iter()
            .collect(),
        app.state
            .worker
            .worker_bridge_processing_state
            .unwrap_or(app.state.processing),
    )
}
