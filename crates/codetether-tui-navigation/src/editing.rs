//! editing navigation policy.
use crate::{Action, Host};
use codetether_tui_models::ViewMode;
use crossterm::event::KeyModifiers;

/// Apply left navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// * `modifiers` — Keyboard modifiers; Control takes priority over Shift where applicable.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_left(app, crossterm::event::KeyModifiers::NONE);
/// # }
/// ```
pub fn handle_left(app: &mut impl Host, modifiers: KeyModifiers) {
    if app.context().view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.apply(Action::CursorWordLeft);
        } else {
            app.apply(Action::CursorLeft);
        }
    }
}

/// Apply right navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// * `modifiers` — Keyboard modifiers; Control takes priority over Shift where applicable.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_right(app, crossterm::event::KeyModifiers::NONE);
/// # }
/// ```
pub fn handle_right(app: &mut impl Host, modifiers: KeyModifiers) {
    if app.context().view_mode == ViewMode::Chat {
        if modifiers.contains(KeyModifiers::CONTROL) {
            app.apply(Action::CursorWordRight);
        } else {
            app.apply(Action::CursorRight);
        }
    }
}

/// Apply delete navigation using current overlay and view state.
///
/// # Arguments
/// * `app` — Native context and operation bindings.
/// # Returns
/// Updates the applicable view in place; unsupported navigation is a no-op.
///
/// # Examples
/// ```no_run
/// # fn navigate(app: &mut impl codetether_tui_navigation::Host) {
/// codetether_tui_navigation::handle_delete(app);
/// # }
/// ```
pub fn handle_delete(app: &mut impl Host) {
    if app.context().view_mode == ViewMode::Chat {
        app.apply(Action::DeleteForward);
    }
}
