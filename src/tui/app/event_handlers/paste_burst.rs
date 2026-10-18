//! Native input projection for shared paste-burst detection.
use crate::tui::{app::state::App, models::ViewMode};
pub(super) fn enter_is_likely_paste_newline(app: &App) -> bool {
    codetether_paste_buffer::burst::enter_is_likely_paste_newline(
        &codetether_paste_buffer::burst::View {
            in_chat: app.state.view_mode == ViewMode::Chat,
            input: &app.state.composer.input,
            cursor: app.state.composer.cursor,
            last_key_at: app.state.last_key_at,
        },
    )
}
