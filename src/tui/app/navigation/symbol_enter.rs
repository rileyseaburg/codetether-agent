//! Bind symbol confirmation to the native composer.
use crate::tui::app::state::App;
/// Confirm the active symbol selection or mention.
pub fn handle(app: &mut App) {
    codetether_symbol_mention::confirm_selection(app);
}
