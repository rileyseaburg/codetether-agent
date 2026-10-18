use super::super::fixtures::ViewState as AppState;
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn older_pages_stop_before_the_expanded_memory_cap() {
    let mut state = AppState::default();
    let boundary = [Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "tail".into(),
        }],
    }];
    state
        .history_page
        .reset("session".into(), &boundary, 1, true);
    let visible =
        codetether_view_retention::CHAT_EXPANDED_MAX_ITEMS - crate::select::PAGE_MESSAGES + 1;

    assert!(!state.history_page.request_older(10, visible));
    assert!(state.history_page.exhausted);
}
