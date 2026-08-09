//! Keyboard scrolling for approval invocation previews.

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::app::state::{App, approval_queue};

pub(in crate::tui::app::event_handlers) fn handle(app: &mut App, key: KeyEvent) -> bool {
    let Some(item) = approval_queue::active() else {
        return false;
    };
    if item.preview.is_none() {
        return false;
    }
    let scroll = &mut app.state.approval_preview_scroll;
    match key.code {
        KeyCode::Up => *scroll = scroll.saturating_sub(1),
        KeyCode::Down => *scroll = scroll.saturating_add(1),
        KeyCode::PageUp => *scroll = scroll.saturating_sub(10),
        KeyCode::PageDown => *scroll = scroll.saturating_add(10),
        KeyCode::Home => *scroll = 0,
        KeyCode::End => {
            // Render clamps to the wrapped-row maximum for the live pane
            // width, so saturating here reaches the true end of content.
            *scroll = u16::MAX;
        }
        _ => return false,
    }
    true
}
