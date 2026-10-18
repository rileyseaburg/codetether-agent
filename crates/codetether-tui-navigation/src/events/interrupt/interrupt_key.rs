//! Priority handling for keys that interrupt an active turn.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::Host;

/// Intercept keys that cancel an active turn.
/// # Arguments
/// * `host` — Current view/processing state and cancellation effects.
/// * `key` — Incoming terminal key event.
/// # Returns
/// Some(false) when consumed for cancellation, otherwise None.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_tui_navigation::events::interrupt::Host) {
/// let _ = codetether_tui_navigation::events::interrupt::handle(host, crossterm::event::KeyEvent::new(crossterm::event::KeyCode::Esc, crossterm::event::KeyModifiers::NONE));
/// # }
/// ```
pub fn handle(host: &mut impl Host, key: KeyEvent) -> Option<bool> {
    if !host.processing() {
        return None;
    }
    let ctrl_c =
        matches!(key.code, KeyCode::Char('c')) && key.modifiers.contains(KeyModifiers::CONTROL);
    let escape_from_chat = matches!(key.code, KeyCode::Esc) && host.in_chat();
    if !ctrl_c && !escape_from_chat {
        return None;
    }
    host.request_cancel();
    host.set_status("Cancellation requested — partial turn will be saved.".to_string());
    Some(false)
}
