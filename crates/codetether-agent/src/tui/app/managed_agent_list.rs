//! Bind roster presentation to the active TUI.
use crate::tui::app::state::App;
pub(crate) fn list(app: &mut App) {
    codetether_agent_command::list(&mut super::super::host::Native { app, parent: None });
}
