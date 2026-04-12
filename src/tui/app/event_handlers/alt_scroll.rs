//! Alt-key scroll bindings for the chat view.
//!
//! Handles Alt-j/k (scroll 1), Alt-d/u (scroll 5),
//! Ctrl-g/G (scroll to top/bottom) while in Chat view mode.
//!
//! # Examples
//!
//! ```ignore
//! if let Some(r) = handle_alt_scroll(&mut app, key) {
//!     return Some(r);
//! }
//! ```

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Try to handle an Alt/Ctrl scroll key in Chat mode.
///
/// Returns `Some(Ok(false))` if the key was consumed, or
/// `None` to fall through to other handlers.
///
/// # Examples
///
/// ```ignore
/// if let Some(r) = handle_alt_scroll(&mut app, key) {
///     return Some(r);
/// }
/// ```
pub(super) fn handle_alt_scroll(app: &mut App, key: KeyEvent) -> Option<anyhow::Result<bool>> {
    if app.state.view_mode != ViewMode::Chat {
        return None;
    }

    let alt = key.modifiers.contains(KeyModifiers::ALT);
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('j') if alt => app.state.scroll_down(1),
        KeyCode::Char('k') if alt => app.state.scroll_up(1),
        KeyCode::Char('d') if alt => app.state.scroll_down(5),
        KeyCode::Char('u') if alt => app.state.scroll_up(5),
        KeyCode::Char('g') if ctrl => app.state.scroll_to_top(),
        KeyCode::Char('G') if ctrl => app.state.scroll_to_bottom(),
        _ => return None,
    }
    Some(Ok(false))
}
