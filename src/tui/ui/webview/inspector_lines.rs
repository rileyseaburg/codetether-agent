//! Native projection for the shared compact inspector.
use crate::tui::app::state::App;
use codetether_inspector_view::compact::View;
use ratatui::text::Line;

pub fn build(app: &App) -> Vec<Line<'static>> {
    codetether_inspector_view::compact::build(&view(app))
}

pub(super) fn view(app: &App) -> View<'_> {
    let state = &app.state;
    View {
        processing: state.processing,
        pending_tool: state.tool_timing.pending_tool_name.as_deref(),
        last_tool: state.tool_timing.last_tool_name.as_deref(),
        model: state.last_completion_model.as_deref(),
        active_agent: state.active_spawned_agent.as_deref(),
        registered: state.worker.worker_bridge_registered_agents.len(),
        context_used: state.context_used,
        context_budget: state.context_budget,
        sync: state.chat_sync.status.as_deref(),
        session_id: state.session_id.as_deref(),
        message_count: state.messages.len(),
        agent_count: state.spawned_agents.len(),
    }
}
