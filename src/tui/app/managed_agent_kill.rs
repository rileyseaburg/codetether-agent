//! Registry removal for one managed child agent.

use crate::session::Session;
use crate::tui::app::state::App;

pub(crate) async fn kill(app: &mut App, session: &Session, rest: &str) {
    let name = rest.trim();
    if name.is_empty() {
        app.state.status = "Usage: /kill <name>".to_string();
        return;
    }
    if crate::tui::app::managed_agent::owned(&session.id, name) {
        let result = crate::tui::app::managed_agent::kill(&session.id, name).await;
        if result.success && app.state.active_spawned_agent.as_deref() == Some(name) {
            app.state.active_spawned_agent = None;
        }
        crate::tui::app::managed_agent::ui::present(
            app,
            &result,
            format!("Terminated @{name}"),
            "Agent removal failed",
        );
        return;
    }
    if app.state.spawned_agents.remove(name).is_some() {
        if app.state.active_spawned_agent.as_deref() == Some(name) {
            app.state.active_spawned_agent = None;
        }
        app.state.streaming_agent_texts.remove(name);
        app.state.status = format!("Removed legacy agent '{name}'");
    } else {
        app.state.status = format!("Agent @{name} is not owned by this session");
    }
}
