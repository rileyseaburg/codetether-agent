//! Ctrl-W side-question composer.

use crate::tui::app::state::App;

pub(super) fn compose_side_question(app: &mut App) {
    if app.state.composer.input.trim().is_empty() {
        app.state.composer.input = "/ask ".to_string();
    } else if !app.state.composer.input.starts_with("/ask") {
        app.state.composer.input = format!("/ask {}", app.state.composer.input.trim());
    }
    app.state.composer.cursor = app.state.composer.input.chars().count();
    app.state.status = "Compose a side question (/ask)".to_string();
    app.state.refresh_slash_suggestions();
}
