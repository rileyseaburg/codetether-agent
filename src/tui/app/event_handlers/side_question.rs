//! Ctrl-W side-question composer.

use crate::tui::app::state::App;

pub(super) fn compose_side_question(app: &mut App) {
    if app.state.input.trim().is_empty() {
        app.state.input = "/ask ".to_string();
    } else if !app.state.input.starts_with("/ask") {
        app.state.input = format!("/ask {}", app.state.input.trim());
    }
    app.state.input_cursor = app.state.input.chars().count();
    app.state.status = "Compose a side question (/ask)".to_string();
    app.state.refresh_slash_suggestions();
}
