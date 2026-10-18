//! Agent focus main policy.
use super::Host;
use crate::Action;

/// Cycle from main chat through every child and back to main chat.
/// Apply the cycle agent focus operation to the current focus ring.
///
/// # Arguments
/// * `app` — Current agent sources and native focus operations.
/// # Returns
/// Updates focus and its related UI state in place.
/// # Examples
/// ```no_run
/// # fn focus(app: &mut impl codetether_tui_navigation::focus::Host) {
/// codetether_tui_navigation::focus::cycle_agent_focus(app);
/// # }
/// ```
pub fn cycle_agent_focus(app: &mut impl Host) {
    let names = super::agent_names(app);
    if names.is_empty() {
        app.apply(Action::Status(
            "No spawned agents. Use /spawn <name> to create one.".to_string(),
        ));
        return;
    }
    let next = match &app.active_agent() {
        None => Some(names[0].clone()),
        Some(current) => names
            .iter()
            .position(|name| name == current)
            .and_then(|index| names.get(index + 1).cloned()),
    };
    super::set_focus(app, next);
}
