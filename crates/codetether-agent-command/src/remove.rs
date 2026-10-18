//! Remove an owned child or a legacy UI child.
use crate::Host;
/// Handle agent removal without allowing cross-parent access.
/// # Arguments
/// `app` supplies ownership and state; `rest` is the requested child name.
/// # Returns
/// Presents the removal result and clears matching focus on success.
/// # Examples
/// ```no_run
/// async fn command<H: codetether_agent_command::Host>(host: &mut H) {
///     codetether_agent_command::kill(host, "worker").await;
/// }
/// ```
pub async fn kill<H: Host>(app: &mut H, rest: &str) {
    let name = rest.trim();
    if name.is_empty() {
        app.status("Usage: /kill <name>".to_string());
        return;
    }
    if app.owned(name) {
        let result = app.remove(name).await;
        if result.success && app.active() == Some(name) {
            app.set_active(None);
        }
        app.present(
            &result,
            format!("Terminated @{name}"),
            "Agent removal failed",
        );
        return;
    }
    if app.remove_legacy(name) {
        if app.active() == Some(name) {
            app.set_active(None);
        }
        app.clear_legacy_output(name);
        app.status(format!("Removed legacy agent '{name}'"));
    } else {
        app.status(format!("Agent @{name} is not owned by this session"));
    }
}
