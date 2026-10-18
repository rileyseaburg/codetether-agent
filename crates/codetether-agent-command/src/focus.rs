//! Focus or message a parent-owned agent.
use crate::{Host, swarm};
/// Handle the operands of an agent focus/message command.
/// # Arguments
/// `app` supplies the parent context; `rest` contains a name and optional message.
/// # Returns
/// Updates focus/status or dispatches the message through the host.
/// # Examples
/// ```no_run
/// async fn command<H: codetether_agent_command::Host>(host: &mut H) {
///     codetether_agent_command::handle(host, "worker").await;
/// }
/// ```
pub async fn handle<H: Host>(app: &mut H, rest: &str) {
    let mut parts = rest.trim().splitn(2, char::is_whitespace);
    let Some(name) = parts.next().filter(|name| !name.is_empty()) else {
        app.status("Usage: /agent <name> [message]".to_string());
        return;
    };
    if swarm::focus(app, name) {
        if parts.next().is_some() {
            app.status("Swarm workers are detail-only; messaging is unavailable".to_string());
        }
        return;
    }
    if !app.owned(name) && !app.legacy_exists(name) {
        app.status(format!("Agent @{name} is not owned by this session"));
        return;
    }
    app.set_active(Some(name.to_string()));
    let Some(message) = parts.next().map(str::trim).filter(|text| !text.is_empty()) else {
        app.status(format!("Focused managed agent: @{name}"));
        return;
    };
    if !app.owned(name) {
        app.status(format!(
            "Legacy agent @{name} cannot accept managed messages"
        ));
        return;
    }
    app.send(name, message).await;
}
