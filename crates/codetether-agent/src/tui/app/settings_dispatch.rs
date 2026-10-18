//! Native entry point for shared settings panel action dispatch.

use crate::session::Session;
use crate::tui::app::state::App;
#[path = "settings_dispatch/host.rs"]
mod host;

/// Execute the action for the currently selected settings row.
pub async fn toggle_selected_setting(app: &mut App, session: &mut Session) {
    let index = app.state.settings_panel.selected;
    codetether_settings_policy::panel::toggle(&mut host::Native { app, session }, index).await;
}
