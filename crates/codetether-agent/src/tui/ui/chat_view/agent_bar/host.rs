//! Native session identity and lazy data sources for the shared header.
use crate::{session::Session, tui::app::state::App};
impl codetether_agent_dashboard::bar::SessionId for Session {
    fn session_id(&self) -> &str {
        &self.id
    }
}
pub(super) fn header(
    app: &App,
) -> impl codetether_agent_dashboard::bar::Host<Session = Session> + '_ {
    codetether_agent_dashboard::bar::Header {
        tree: &app.state.agent_tree,
        subtasks: &app.state.swarm.subtasks,
        session_id: app.state.session_id.as_deref(),
        processing: app.state.processing,
        tools: || {
            app.state
                .session_id
                .as_deref()
                .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
                .unwrap_or_else(crate::tool::agent::bridge::list_agent_tool_agents)
        },
        model: || crate::tui::app::spawn_agent::model::current_model_id(app),
    }
}
