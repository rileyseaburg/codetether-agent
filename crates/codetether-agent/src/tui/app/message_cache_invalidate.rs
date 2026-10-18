//! Explicit invalidation for cached chat render lines.

use crate::tui::app::state::AppState;

pub fn clear(state: &mut AppState) {
    state.message_cache.clear();
}
