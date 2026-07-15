//! Session-switch and moved-viewport regression tests.

use crate::tui::app::state::AppState;

#[test]
fn stale_generation_result_is_ignored() {
    let mut state = AppState::default();
    let stale = state.history_page.generation;
    state
        .history_page
        .reset("new-session".into(), &[], 0, false);
    state
        .history_page
        .tx
        .send((stale, Err("old session".into())))
        .unwrap();
    assert!(state.history_page.take_result().is_none());
}

#[test]
fn prepend_preserves_view_when_user_moved_down() {
    let mut state = AppState::default();
    state.history_page.pending_old_lines = Some(100);
    state.history_page.pending_old_scroll = 20;
    state.apply_pending_history_anchor(160);
    assert_eq!(state.chat_scroll, 80);
}
