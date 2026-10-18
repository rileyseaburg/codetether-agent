//! Mouse-wheel scrolling routed to the visible approval details.

use super::Host;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Consume the wheel only when approval keyboard scrolling would be active.
/// Scroll the visible approval preview by wheel steps.
/// # Arguments
/// * `host` — Frontend state and command bindings.
/// * `direction` — Navigation key produced by the wheel.
/// * `amount` — Number of scroll steps requested.
/// # Returns
/// Whether the action was consumed.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_approval_view::interactive::Host) {
/// let _ = codetether_approval_view::interactive::wheel(host, crossterm::event::KeyCode::Down, 3);
/// # }
/// ```
pub fn handle(host: &mut impl Host, direction: KeyCode, amount: usize) -> bool {
    let key = KeyEvent::new(direction, KeyModifiers::NONE);
    if !super::scroll(host, key) {
        return false;
    }
    for _ in 1..amount {
        super::scroll(host, key);
    }
    true
}
