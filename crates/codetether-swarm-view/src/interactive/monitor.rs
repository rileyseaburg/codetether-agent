//! Bare `/swarm` command: open the monitor or show launch guidance.

use super::View;

/// Handle the bare `/swarm` command.
///
/// If a swarm is already running, opens the live monitor view. Otherwise
/// shows usage guidance in chat instead of a confusing empty monitor — the
/// monitor opens automatically once `/swarm <task>` launches a run.
///
/// # Arguments
/// * `app` — View receiving monitor selection or launch guidance.
///
/// # Returns
/// Updates the supplied view in place.
///
/// # Examples
/// ```no_run
/// # fn show(view: &mut impl codetether_swarm_view::interactive::View) {
/// codetether_swarm_view::interactive::open_swarm_monitor(view);
/// # }
/// ```
pub fn open_swarm_monitor(app: &mut impl View) {
    if app.swarm().active {
        app.show_monitor();
        app.set_status("Swarm monitor".to_string());
        return;
    }
    app.set_status("Usage: /swarm <task> to launch a parallel swarm".to_string());
    app.push_system(
        "No swarm is running. To launch one, type:\n\
         /swarm <task description>\n\
         Example: /swarm research the auth module, then propose tests\n\
         The monitor opens automatically once a swarm starts."
            .to_string(),
    );
    app.scroll_to_bottom();
}
