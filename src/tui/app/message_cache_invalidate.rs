//! Explicit invalidation for cached chat render lines.

use crate::tui::app::state::AppState;

pub fn clear(state: &mut AppState) {
    state.cached_message_lines.clear();
    state.cached_messages_len = 0;
    state.cached_frozen_len = 0;
}
