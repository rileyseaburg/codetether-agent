//! Bind shared autochat event handling to native app state.
use super::events::AutochatUiEvent;
use crate::tui::app::state::AppState;
/// Apply one relay event to the app.
pub fn handle_autochat_event(app: &mut AppState, event: AutochatUiEvent) {
    codetether_autochat_core::interactive::handler::handle_autochat_event(app, event);
}
