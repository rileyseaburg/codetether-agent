//! Native bindings for keyboard navigation and agent focus.
use crate::tui::app::state::App;
#[cfg(test)]
use crate::tui::models::ViewMode;
use crossterm::event::KeyModifiers;
#[path = "navigation/agent_focus.rs"]
mod agent_focus;
#[path = "navigation/host.rs"]
mod host;
pub(crate) use host::Navigation;
#[path = "navigation/symbol_enter.rs"]
mod symbol_enter;
#[cfg(test)]
mod tests;
pub use agent_focus::{cycle_agent_focus, cycle_agent_focus_back, handle_tab};
pub use symbol_enter::handle as handle_symbol_enter;
pub fn handle_escape(app: &mut App) {
    codetether_tui_navigation::handle_escape(&mut host::Navigation(app));
}
pub fn toggle_help(app: &mut App) {
    codetether_tui_navigation::toggle_help(&mut host::Navigation(app));
}
pub fn handle_up(app: &mut App, modifiers: KeyModifiers) {
    codetether_tui_navigation::handle_up(&mut host::Navigation(app), modifiers);
}
pub fn handle_down(app: &mut App, modifiers: KeyModifiers) {
    codetether_tui_navigation::handle_down(&mut host::Navigation(app), modifiers);
}
pub fn handle_page_up(app: &mut App) {
    codetether_tui_navigation::handle_page_up(&mut host::Navigation(app));
}
pub fn handle_page_down(app: &mut App) {
    codetether_tui_navigation::handle_page_down(&mut host::Navigation(app));
}
pub fn handle_left(app: &mut App, modifiers: KeyModifiers) {
    codetether_tui_navigation::handle_left(&mut host::Navigation(app), modifiers);
}
pub fn handle_right(app: &mut App, modifiers: KeyModifiers) {
    codetether_tui_navigation::handle_right(&mut host::Navigation(app), modifiers);
}
pub fn handle_delete(app: &mut App) {
    codetether_tui_navigation::handle_delete(&mut host::Navigation(app));
}
pub fn handle_home(app: &mut App) {
    codetether_tui_navigation::handle_home(&mut host::Navigation(app));
}
pub fn handle_end(app: &mut App) {
    codetether_tui_navigation::handle_end(&mut host::Navigation(app));
}
