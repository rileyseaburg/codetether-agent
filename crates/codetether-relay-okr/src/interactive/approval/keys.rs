//! Keyboard routing for the pending `/go` OKR approval modal.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::Host;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Approve,
    Deny,
}

fn action(key: KeyEvent) -> Option<Action> {
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
    {
        return None;
    }
    match key.code {
        KeyCode::Char('a' | 'A') => Some(Action::Approve),
        KeyCode::Char('d' | 'D') => Some(Action::Deny),
        _ => None,
    }
}

/// Handle unmodified A/D keys while an objective approval is pending.
/// # Arguments
/// * `host` — Pending approval and frontend effects.
/// * `key` — Incoming terminal key.
/// # Returns
/// Whether an approve or deny action consumed the key.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_relay_okr::interactive::approval::Host, key: crossterm::event::KeyEvent) {
/// let _ = codetether_relay_okr::interactive::approval::handle(host, key).await;
/// # }
/// ```
pub async fn handle(host: &mut impl Host, key: KeyEvent) -> bool {
    if host.pending().is_none() {
        return false;
    }
    match action(key) {
        Some(Action::Approve) => super::actions::handle_okr_approve(host).await,
        Some(Action::Deny) => super::actions::handle_okr_deny(host).await,
        None => return false,
    }
    true
}

/// Test whether a key represents an objective approval decision.
/// # Arguments
/// * `key` — Incoming terminal key.
/// # Returns
/// True for A/D without Control or Alt modifiers.
/// # Examples
/// ```
/// assert!(codetether_relay_okr::interactive::approval::is_action(crossterm::event::KeyCode::Char('a').into()));
/// ```
pub fn is_action(key: KeyEvent) -> bool {
    action(key).is_some()
}
