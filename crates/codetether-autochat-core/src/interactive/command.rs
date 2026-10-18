//! Validate and start an interactive autochat relay.
use super::host::{Host, View};
/// Start a relay for a nonempty task when no relay is running.
///
/// # Arguments
/// * `app` — Relay state, model selection and status presentation.
/// * `rest` — Task text following `/autochat`.
///
/// # Returns
/// Updates the view in place; empty tasks and active runs report a status.
///
/// # Examples
/// ```no_run
/// # fn launch<H: codetether_autochat_core::interactive::host::Host, E: Send + 'static>(
/// #     view: &mut impl codetether_autochat_core::interactive::host::View<E>) {
/// codetether_autochat_core::interactive::command::handle_autochat_command::<H, E>(
///     view, "review the implementation",
/// );
/// # }
/// ```
pub fn handle_autochat_command<H: Host, E: Send + 'static>(app: &mut impl View<E>, rest: &str) {
    let task = rest.trim().to_string();
    if task.is_empty() {
        app.set_status("Usage: /autochat <task description>".to_string());
        return;
    }
    if app.autochat().running {
        app.set_status("Autochat relay already running.".to_string());
        return;
    }
    let model = app.last_completion_model().unwrap_or_default();
    let rx = super::worker::start_autochat_relay::<H, E>(task, model);
    app.autochat().running = true;
    app.autochat().rx = Some(rx);
    app.set_status("Autochat relay started.".to_string());
}
