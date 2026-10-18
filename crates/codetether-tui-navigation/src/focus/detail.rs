//! Agent focus detail policy.
use super::Host;
use crate::Action;

/// Apply the cycle detail operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::cycle_detail(app);
/// # }
/// ```
pub fn cycle(app: &mut impl Host) {
    super::child::next(app);
    open_swarm_detail(app);
    app.apply(Action::SubagentScroll(0));
}

pub(super) fn cycle_back(app: &mut impl Host) {
    super::child::previous(app);
    open_swarm_detail(app);
    app.apply(Action::SubagentScroll(0));
}

fn open_swarm_detail(app: &mut impl Host) {
    let selected = app.selected_swarm_agent();
    if selected.as_deref() == app.active_agent().as_deref() {
        app.apply(Action::SubagentDetail(false));
        app.show_swarm();
        app.enter_swarm_detail();
    }
}
