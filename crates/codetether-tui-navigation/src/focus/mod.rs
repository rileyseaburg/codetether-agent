//! Agent focus rings, detail transitions and focus-change effects.
//!
//! Implement [Host] for agent lookup and UI operations. [handle_tab] accepts
//! completion first, then selects the applicable focus ring. Main-chat and
//! child-only cycling retain their distinct wrap behavior.
mod back;
mod child;
mod detail;
mod host;
mod main;
mod names;
mod set;
mod swarm;
use crate::Action;
pub use back::cycle_agent_focus_back;
use codetether_tui_models::ViewMode;
pub use detail::cycle as cycle_detail;
pub use host::Host;
pub use main::cycle_agent_focus;
pub use names::assemble;
use set::set_focus;
fn agent_names(app: &impl Host) -> Vec<String> {
    app.agent_names()
}

/// Accept a slash suggestion or cycle the applicable focus ring.
/// Apply the handle tab operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::handle_tab(app);
/// # }
/// ```
pub fn handle_tab(app: &mut impl Host) {
    if app.apply_selected_slash_suggestion() {
        app.apply(Action::Status("Command autocompleted".to_string()));
    } else if app.context().view_mode == ViewMode::Subagents {
        if app.context().subagent_detail {
            detail::cycle(app);
        } else {
            child::next(app);
        }
        app.apply(Action::SubagentScroll(0));
    } else {
        cycle_agent_focus(app);
    }
}

/// Apply the cycle child focus operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::cycle_child_focus(app);
/// # }
/// ```
pub fn cycle_child_focus(app: &mut impl Host) {
    child::next(app);
}

/// Apply the cycle child focus back operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::cycle_child_focus_back(app);
/// # }
/// ```
pub fn cycle_child_focus_back(app: &mut impl Host) {
    child::previous(app);
}
