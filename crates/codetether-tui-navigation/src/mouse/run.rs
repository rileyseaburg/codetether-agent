//! Mouse wheel and left-click dispatch with original overlay priority.
use super::{
    Host,
    overlay_scroll::{scroll_overlay_down, scroll_overlay_up},
    scroll_down::scroll_down_by_mode,
    scroll_up::scroll_up_by_mode,
};
use crate::Action;
use crossterm::event::{KeyCode, MouseButton, MouseEvent, MouseEventKind};
use std::path::Path;
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
/// ```no_run
/// # fn scroll(host: &mut impl codetether_tui_navigation::mouse::Host) {
/// codetether_tui_navigation::mouse::scroll_mouse_up(host);
/// # }
/// ```
pub fn scroll_mouse_up(host: &mut impl Host) {
    if host.context().show_help {
        host.apply(Action::HelpUp(AMOUNT));
        return;
    }
    if host.approval_wheel(KeyCode::Up, AMOUNT) || scroll_overlay_up(host, AMOUNT) {
        return;
    }
    scroll_up_by_mode(host, AMOUNT);
}

/// Handle one mouse-scroll-down event.
///
/// Mirror of [`scroll_mouse_up`] for the downward direction.
///
/// # Examples
///
/// ```no_run
/// # fn scroll(host: &mut impl codetether_tui_navigation::mouse::Host) {
/// codetether_tui_navigation::mouse::scroll_mouse_down(host);
/// # }
/// ```
pub fn scroll_mouse_down(host: &mut impl Host) {
    if host.context().show_help {
        host.apply(Action::HelpDown(AMOUNT, 200));
        return;
    }
    if host.approval_wheel(KeyCode::Down, AMOUNT) || scroll_overlay_down(host, AMOUNT) {
        return;
    }
    scroll_down_by_mode(host, AMOUNT);
}

/// Dispatch a mouse event (scroll wheel and left-click to open files).
///
/// Routes `ScrollUp`/`ScrollDown` to view-mode-aware scroll handlers, and a
/// left button press in the Chat view to [`Host::click_open_path`].
///
/// # Examples
///
/// ```no_run
/// # async fn dispatch(host: &mut impl codetether_tui_navigation::mouse::Host, cwd: &std::path::Path, mouse: crossterm::event::MouseEvent) {
/// codetether_tui_navigation::mouse::handle_mouse_event(host, cwd, mouse).await;
/// # }
/// ```
pub async fn handle_mouse_event(host: &mut impl Host, cwd: &Path, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => scroll_mouse_up(host),
        MouseEventKind::ScrollDown => scroll_mouse_down(host),
        MouseEventKind::Down(MouseButton::Left) => {
            if host.editor_click(cwd, mouse.column, mouse.row).await {
                return;
            }
            host.click_open_path(cwd, mouse.column, mouse.row);
        }
        _ => {}
    }
}
