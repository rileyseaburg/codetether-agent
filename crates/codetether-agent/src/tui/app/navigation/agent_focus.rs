//! Native bindings for shared agent focus policy.
use crate::tui::app::state::App;
#[cfg(test)]
#[path = "agent_focus_detail.rs"]
mod detail;
#[path = "agent_focus_names.rs"]
mod names;
pub(super) use names::agent_names;
pub fn handle_tab(app: &mut App) {
    codetether_tui_navigation::focus::handle_tab(&mut super::host::Navigation(app));
}
pub fn cycle_agent_focus(app: &mut App) {
    codetether_tui_navigation::focus::cycle_agent_focus(&mut super::host::Navigation(app));
}
pub fn cycle_agent_focus_back(app: &mut App) {
    codetether_tui_navigation::focus::cycle_agent_focus_back(&mut super::host::Navigation(app));
}
pub(super) fn cycle_child_focus(app: &mut App) {
    codetether_tui_navigation::focus::cycle_child_focus(&mut super::host::Navigation(app));
}
pub(super) fn cycle_child_focus_back(app: &mut App) {
    codetether_tui_navigation::focus::cycle_child_focus_back(&mut super::host::Navigation(app));
}
