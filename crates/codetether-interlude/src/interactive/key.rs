//! Application redraw adapter for Tether Catch keys.
use crate::InterludeState;
use crossterm::event::KeyEvent;
/// Apply a key when a play break is active.
/// # Arguments
/// * `state` — Optional active game.
/// * `redraw` — Frontend redraw flag.
/// * `key` — Incoming terminal key.
/// # Returns
/// Whether the game handled the key.
/// # Examples
/// ```
/// let (mut state, mut redraw) = (None, false);
/// let key = crossterm::event::KeyCode::Left.into();
/// assert!(!codetether_interlude::interactive::handle(&mut state, &mut redraw, key));
/// assert!(!redraw);
/// ```
pub fn handle(state: &mut Option<InterludeState>, redraw: &mut bool, key: KeyEvent) -> bool {
    let Some(game) = state.as_mut() else {
        return false;
    };
    if !crate::handle_key(game, key) {
        return false;
    }
    *redraw = true;
    true
}
