//! Session event pipeline helpers.

use crate::tui::app::state::App;

use super::retention;

pub(super) fn stop(app: &mut App) {
    retention::trim(app);
}
