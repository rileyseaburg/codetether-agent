//! Mouse-wheel scrolling routed to the visible approval details.

use crate::tui::app::state::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Consume the wheel only when approval keyboard scrolling would be active.
pub(in crate::tui::app::event_handlers) fn handle(
    app: &mut App,
    direction: KeyCode,
    amount: usize,
) -> bool {
    let key = KeyEvent::new(direction, KeyModifiers::NONE);
    if !super::scroll_key::handle(app, key) {
        return false;
    }
    for _ in 1..amount {
        super::scroll_key::handle(app, key);
    }
    true
}
