//! Native registry specialization of shared local-agent tabs.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Append local-agent tabs in depth-first order.
pub fn push_tui_agents(spans: &mut Vec<Span<'static>>, app: &App, active: Option<&str>) {
    codetether_agent_dashboard::bar::push_tui_agents(spans, &app.state.agent_tree.spawned, active);
}
