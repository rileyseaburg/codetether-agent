//! Native projection into the shared scrollable child transcript panel.
use crate::tui::app::state::AppState;
use ratatui::{Frame, layout::Rect};
pub(super) fn render(f: &mut Frame, area: Rect, state: &AppState) {
    codetether_agent_dashboard::render::detail(
        f,
        area,
        state.active_spawned_agent.as_deref(),
        state.subagent_detail_scroll,
        super::subagent_detail_lines::lines(state),
    );
}
