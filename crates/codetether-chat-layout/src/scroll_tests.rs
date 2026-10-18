//! Manual scrolling and follow-latest regression coverage.
use crate::ChatScrollState;

#[test]
fn scroll_up_from_follow_latest_enters_manual_scroll_mode() {
    let mut state = ChatScrollState::default();
    state.set_chat_max_scroll(25);
    state.scroll_to_bottom();
    assert_eq!(state.scroll_up(1), None);
    assert_eq!(state.chat_scroll, 24);
}

#[test]
fn scroll_down_returns_to_follow_latest_at_bottom() {
    let mut state = ChatScrollState::default();
    state.set_chat_max_scroll(25);
    state.chat_scroll = 24;
    state.scroll_down(1);
    assert_eq!(state.chat_scroll, 1_000_000);
}
