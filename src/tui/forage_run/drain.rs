//! Bind forage update draining to the application chat view.
use crate::tui::app::state::App;
/// Apply pending forage updates and report whether the view changed.
pub fn drain_forage_updates(app: &mut App) -> bool {
    codetether_forage_interactive::drain_forage_updates(app)
}
