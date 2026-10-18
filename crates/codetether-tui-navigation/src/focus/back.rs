//! Agent focus back policy.
use super::Host;
use super::{agent_names, child, detail, set_focus};
use crate::Action;
use codetether_tui_models::ViewMode;

/// Cycle the applicable focus ring backward by one slot.
/// Apply the cycle agent focus back operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::cycle_agent_focus_back(app);
/// # }
/// ```
pub fn cycle_agent_focus_back(app: &mut impl Host) {
    if app.context().view_mode == ViewMode::Subagents {
        if app.context().subagent_detail {
            detail::cycle_back(app);
        } else {
            child::previous(app);
        }
        return;
    }
    let names = agent_names(app);
    if names.is_empty() {
        app.apply(Action::Status(
            "No spawned agents. Use /spawn <name> to create one.".to_string(),
        ));
        return;
    }
    let previous = match &app.active_agent() {
        None => Some(names[names.len() - 1].clone()),
        Some(current) => match names.iter().position(|name| name == current) {
            Some(0) | None => None,
            Some(index) => Some(names[index - 1].clone()),
        },
    };
    set_focus(app, previous);
}
