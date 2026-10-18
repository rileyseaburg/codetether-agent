//! Detach validation, persistence and presentation order.
use super::{Host, name::pick_name, register::register_detached};
use codetether_session_fork::ForkSession;
/// Handle `/detach [name]`: snapshot the current thread into a backgrounded
/// sub-agent that keeps running inside the TUI.
/// # Arguments
/// Receives the frontend, current session and optional preferred agent name.
/// # Returns
/// Reports failures through status; successful snapshots are registered and shown.
/// # Examples
/// ```no_run
/// async fn detach<H: codetether_agent_command::detach::Host>(host: &mut H, session: &mut H::Session) {
///     codetether_agent_command::detach::handle(host, session, "background").await;
/// }
/// ```
pub async fn handle<H: Host>(app: &mut H, session: &mut H::Session, rest: &str) {
    if app.is_processing() {
        app.push_system("Cannot detach while a response is in progress. Press Esc first.".into());
        return;
    }
    if let Err(error) = session.save().await {
        app.set_status(format!("Detach aborted: failed to save session: {error}"));
        return;
    }

    let name = pick_name(app, rest);
    let child =
        match codetether_session_fork::detached::create(session, || app.resume_window()).await {
            Ok(c) => c,
            Err(err) => {
                app.set_status(format!("Detach failed: {err}"));
                return;
            }
        };
    let child_id = child.id().to_string();
    register_detached(app, &name, child);
    app.set_status(format!("Detached thread → subagent '{name}'"));
    app.show_subagents();
    app.push_system(
        format!(
            "Detached this thread into subagent '{name}' ({}).\n  Open /agents for the all-child dashboard, or switch with Tab / /focus {name}.",
            &child_id[..8.min(child_id.len())]
        ),
    );
}
