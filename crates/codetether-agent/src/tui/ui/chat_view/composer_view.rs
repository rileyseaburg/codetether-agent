//! Application projection for the shared composer renderer.
use crate::tui::{app::state::App, models::InputMode};
pub(super) fn projection(app: &App) -> codetether_composer_view::ComposerView<'_> {
    codetether_composer_view::ComposerView {
        text: &app.state.composer.input,
        cursor: app.state.composer.cursor,
        processing: app.state.processing,
        command_mode: matches!(app.state.composer.mode, InputMode::Command),
    }
}
