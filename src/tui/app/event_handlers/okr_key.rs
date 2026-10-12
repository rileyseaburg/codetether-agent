//! Keyboard routing for the pending `/go` OKR approval modal.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::state::App;

#[cfg(test)]
#[path = "okr_key_tests.rs"]
mod tests;

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

pub async fn handle(app: &mut App, key: KeyEvent) -> bool {
    if app.state.pending_okr_approval.is_none() {
        return false;
    }
    match action(key) {
        Some(Action::Approve) => super::handle_okr_approve(app).await,
        Some(Action::Deny) => super::handle_okr_deny(app).await,
        None => return false,
    }
    true
}

#[cfg(test)]
pub(super) fn is_action(key: KeyEvent) -> bool {
    action(key).is_some()
}
