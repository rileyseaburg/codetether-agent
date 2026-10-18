//! Network-access toggle for the Settings panel.

use crate::session::Session;
use crate::tui::app::state::App;

pub use codetether_settings_policy::network_access_status_message;

pub async fn set_network_access(app: &mut App, session: &mut Session, next: bool) {
    crate::tui::app::settings::host::set(
        app,
        session,
        crate::tui::app::settings::host::Preference::Network,
        next,
    )
    .await;
}

pub async fn toggle_network_access(app: &mut App, session: &mut Session) {
    set_network_access(app, session, !app.state.preferences.allow_network).await;
}
