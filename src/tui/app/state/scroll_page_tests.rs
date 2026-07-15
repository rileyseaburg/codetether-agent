//! Tests for viewport-relative chat paging.

use super::super::AppState;

#[test]
fn page_uses_viewport_with_one_context_line() {
    let mut state = AppState::default();
    state.set_chat_max_scroll(100);
    state.history_page.set_viewport_height(20);
    state.scroll_to_bottom();
    state.page_up();
    assert_eq!(state.chat_scroll, 81);
}

#[test]
fn page_down_restores_follow_latest() {
    let mut state = AppState::default();
    state.set_chat_max_scroll(100);
    state.history_page.set_viewport_height(20);
    state.chat_scroll = 90;
    state.page_down();
    assert!(state.chat_scroll >= crate::tui::constants::SCROLL_BOTTOM);
    assert!(state.chat_auto_follow);
}
