//! Network-access toggle for the Settings panel.

use crate::session::Session;
use crate::tui::app::state::App;

use super::{on_off_label, persist};

pub fn network_access_status_message(enabled: bool) -> String {
    format!("TUI network access: {}", on_off_label(enabled))
}

pub async fn set_network_access(app: &mut App, session: &mut Session, next: bool) {
    app.state.allow_network = next;
    session.metadata.allow_network = next;
    if next {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1") }
    } else {
        unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") }
    }
    persist(app, session, network_access_status_message(next)).await;
}

pub async fn toggle_network_access(app: &mut App, session: &mut Session) {
    set_network_access(app, session, !app.state.allow_network).await;
}
