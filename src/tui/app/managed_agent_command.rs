//! `/agent` focus and direct-message command.

use crate::session::Session;
use crate::tui::app::state::App;

#[path = "managed_agent_admin.rs"]
pub(crate) mod admin;

pub(crate) async fn handle(app: &mut App, session: &Session, rest: &str) {
    let mut parts = rest.trim().splitn(2, char::is_whitespace);
    let Some(name) = parts.next().filter(|name| !name.is_empty()) else {
        app.state.status = "Usage: /agent <name> [message]".to_string();
        return;
    };
    if !crate::tui::app::managed_agent::owned(&session.id, name)
        && !app.state.spawned_agents.contains_key(name)
    {
        app.state.status = format!("Agent @{name} is not owned by this session");
        return;
    }
    app.state.active_spawned_agent = Some(name.to_string());
    let Some(message) = parts.next().map(str::trim).filter(|text| !text.is_empty()) else {
        app.state.status = format!("Focused managed agent: @{name}");
        return;
    };
    if !crate::tui::app::managed_agent::owned(&session.id, name) {
        app.state.status = format!("Legacy agent @{name} cannot accept managed messages");
        return;
    }
    super::chat::send(app, &session.id, name, message).await;
}
