//! Paging order, boundary, and rendered-anchor tests.

use crate::tui::app::state::AppState;

#[path = "limit_tests.rs"]
mod limit_tests;

#[test]
fn prepend_anchor_moves_back_by_requested_rows() {
    let mut state = AppState::default();
    state.history_page.pending_old_lines = Some(100);
    state.history_page.pending_old_scroll = 0;
    state.history_page.pending_render_rewind = 19;
    state.apply_pending_history_anchor(160);
    assert_eq!(state.chat_viewport.chat_scroll, 41);
    assert!(!state.chat_viewport.chat_auto_follow);
}
