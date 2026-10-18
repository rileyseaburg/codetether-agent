//! Bind preview keyboard handling to current native UI state.
use crate::tui::{app::state::App, models::ViewMode};
use crossterm::event::KeyEvent;
pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, key: KeyEvent) -> bool {
    codetether_approval_view::input::handle(
        codetether_approval_view::input::Preview {
            chat: app.state.view_mode == ViewMode::Chat,
            editing: app.state.approval_edit.is_some(),
            help: app.state.show_help,
            input: &app.state.input,
            scroll: &mut app.state.approval_preview_scroll,
        },
        key,
    )
}
