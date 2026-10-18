//! Bind the parent session and native registry to tool-child detail formatting.
mod host;
use crate::tui::app::state::AppState;
use ratatui::text::Line;
pub(super) fn lines(state: &AppState, name: &str) -> Vec<Line<'static>> {
    codetether_agent_dashboard::tool_detail::lines(
        &host::Registry,
        state.session_id.as_deref(),
        name,
    )
}
