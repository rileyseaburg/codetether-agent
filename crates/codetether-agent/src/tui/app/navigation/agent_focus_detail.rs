//! Application integration test binding for shared detail cycling.
use crate::tui::app::state::App;
fn cycle(app: &mut App) {
    codetether_tui_navigation::focus::cycle_detail(&mut super::super::host::Navigation(app));
}
#[cfg(test)]
#[path = "agent_focus_detail_tests.rs"]
mod tests;
