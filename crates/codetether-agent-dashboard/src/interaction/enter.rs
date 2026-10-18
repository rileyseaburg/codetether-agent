//! Choose the detail destination for dashboard Enter.
use super::{Host, focus::discard_stale_focus, swarm};
/// Open the focused child, a swarm detail view, or the next available agent.
/// # Arguments
/// Receives the frontend's current selection and native navigation operations.
/// # Returns
/// Updates detail selection and status; an empty dashboard reports no agents.
/// # Examples
/// ```no_run
/// fn open<H: codetether_agent_dashboard::interaction::Host>(host: &mut H) {
///     codetether_agent_dashboard::interaction::enter(host);
/// }
/// ```
pub fn enter(app: &mut impl Host) {
    discard_stale_focus(app);
    if let Some(name) = app.selected().map(str::to_owned) {
        if swarm::open(app, &name) {
            return;
        }
        app.open_local_detail();
        app.set_status(format!("Agent @{name} detail"));
    } else if !app.swarm().subtasks.is_empty() {
        app.show_swarm();
        app.swarm_mut().enter_detail();
        app.set_status("Swarm agent detail".to_string());
    } else {
        app.cycle_focus();
        if app.selected().is_some() {
            enter(app);
        } else {
            app.set_status("No agents to inspect".to_string());
        }
    }
}
