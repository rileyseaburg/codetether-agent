//! Mouse event dispatch for the TUI.
//!
//! Routes scroll-wheel events through overlay checks then
//! delegates to view-mode-aware scroll helpers.
//!
//! # Examples
//!
//! ```ignore
//! scroll_mouse_up(&mut app);
//! scroll_mouse_down(&mut app);
//! ```

use crossterm::event::{MouseEvent, MouseEventKind};

use crate::tui::app::state::App;

use super::overlay_scroll::{scroll_overlay_down, scroll_overlay_up};
use super::scroll_down::scroll_down_by_mode;
use super::scroll_up::scroll_up_by_mode;

/// Number of rows to scroll per mouse wheel tick.
const AMOUNT: usize = 3;

/// Handle one mouse-scroll-up event.
///
/// Checks overlay state first (help, symbol search, etc.)
/// and only falls through to the view-mode scroll when no
/// overlay consumed the event.
///
/// # Examples
///
/// ```ignore
/// scroll_mouse_up(&mut app);
/// ```
pub(super) fn scroll_mouse_up(app: &mut App) {
    if app.state.show_help {
        app.state.help_scroll.scroll_up(AMOUNT);
        return;
    }
    if scroll_overlay_up(app, AMOUNT) {
        return;
    }
    scroll_up_by_mode(app, AMOUNT);
}

/// Handle one mouse-scroll-down event.
///
/// Mirror of [`scroll_mouse_up`] for the downward direction.
///
/// # Examples
///
/// ```ignore
/// scroll_mouse_down(&mut app);
/// ```
pub(super) fn scroll_mouse_down(app: &mut App) {
    if app.state.show_help {
        app.state.help_scroll.scroll_down(AMOUNT, 200);
        return;
    }
    if scroll_overlay_down(app, AMOUNT) {
        return;
    }
    scroll_down_by_mode(app, AMOUNT);
}

/// Dispatch a mouse event (currently only scroll wheel).
///
/// Routes `ScrollUp` and `ScrollDown` to the appropriate
/// view-mode-aware scroll handler.
///
/// # Examples
///
/// ```ignore
/// handle_mouse_event(&mut app, mouse);
/// ```
pub fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => scroll_mouse_up(app),
        MouseEventKind::ScrollDown => scroll_mouse_down(app),
        _ => {}
    }
}
