//! Keyboard scrolling for approval invocation previews.

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::state::{App, approval_queue};

pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, key: KeyEvent) -> bool {
    if app.state.view_mode != crate::tui::models::ViewMode::Chat
        || app.state.approval_edit.is_some()
        || app.state.show_help
        || approval_queue::feedback_input(&app.state.input)
    {
        return false;
    }
    let Some(item) = approval_queue::active() else {
        return false;
    };
    if matches!(key.code, KeyCode::Tab | KeyCode::BackTab) {
        approval_queue::cycle(
            key.code == KeyCode::Tab
                && !key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::SHIFT),
        );
        app.state.approval_preview_scroll = 0;
        return true;
    }
    let scroll = &mut app.state.approval_preview_scroll;
    *scroll = (*scroll).min(item.scroll_limit);
    match key.code {
        KeyCode::Up => *scroll = scroll.saturating_sub(1),
        KeyCode::Down => *scroll = scroll.saturating_add(1).min(item.scroll_limit),
        KeyCode::PageUp => *scroll = scroll.saturating_sub(10),
        KeyCode::PageDown => *scroll = scroll.saturating_add(10).min(item.scroll_limit),
        KeyCode::Home => *scroll = 0,
        KeyCode::End => *scroll = item.scroll_limit,
        _ => return false,
    }
    true
}
