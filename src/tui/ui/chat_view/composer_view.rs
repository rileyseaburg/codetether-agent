//! Application projection for the shared composer renderer.
use crate::tui::{app::state::App, models::InputMode};
pub(super) fn projection(app: &App) -> codetether_composer_view::ComposerView<'_> {
    codetether_composer_view::ComposerView {
        text: &app.state.input,
        cursor: app.state.input_cursor,
        processing: app.state.processing,
        command_mode: matches!(app.state.input_mode, InputMode::Command),
    }
}
