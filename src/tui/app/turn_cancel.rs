//! In-flight turn cancellation handle helpers.

use crate::tui::app::state::App;

pub fn clear(app: &mut App) {
    app.state.current_turn_cancel = None;
}
