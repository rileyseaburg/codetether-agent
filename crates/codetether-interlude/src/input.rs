//! Local movement keys for the play break.
use crate::InterludeState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// Apply a movement key, leaving control and alt combinations to the host.
///
/// # Arguments
/// * `game` - Active game state.
/// * `key` - Terminal key event.
///
/// # Returns
/// Whether a movement command was consumed.
///
/// # Examples
/// ```
/// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
/// let mut game = codetether_interlude::InterludeState::new();
/// assert!(codetether_interlude::handle_key(&mut game, KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)));
/// ```
pub fn handle_key(game: &mut InterludeState, key: KeyEvent) -> bool {
    if key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
    {
        return false;
    }
    match key.code {
        KeyCode::Left | KeyCode::Char('a') => game.left(),
        KeyCode::Right | KeyCode::Char('d') => game.right(),
        _ => return false,
    }
    true
}
