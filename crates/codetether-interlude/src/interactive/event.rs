//! Play-break session event adapter.

use crate::InterludeState;
use codetether_session_events::SessionEvent;

/// Apply a runtime play-break event to the optional game.
/// # Arguments
/// * `state` — Optional active game.
/// * `redraw` — Frontend redraw flag.
/// * `evt` — Runtime event; unrelated variants pass through.
/// # Returns
/// None for play-break events, otherwise the original event.
/// # Examples
/// ```
/// let (mut state, mut redraw) = (None, false);
/// let event = codetether_session_events::SessionEvent::<()>::PlayBreak(false);
/// assert!(codetether_interlude::interactive::handle_event(&mut state, &mut redraw, event).is_none());
/// assert!(redraw);
/// ```
pub fn handle_event<S>(
    state: &mut Option<InterludeState>,
    redraw: &mut bool,
    evt: SessionEvent<S>,
) -> Option<SessionEvent<S>> {
    match evt {
        SessionEvent::PlayBreak(true) => *state = Some(InterludeState::new()),
        SessionEvent::PlayBreak(false) => *state = None,
        other => return Some(other),
    }
    *redraw = true;
    None
}
