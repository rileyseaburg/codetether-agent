use super::retained_limit;
use crate::tui::retained_payload::{CHAT_EXPANDED_MAX_ITEMS, CHAT_RETAINED_MAX_ITEMS};

#[test]
fn expanded_history_remains_bounded() {
    assert_eq!(retained_limit(false), CHAT_RETAINED_MAX_ITEMS);
    assert_eq!(retained_limit(true), CHAT_EXPANDED_MAX_ITEMS);
    assert!(CHAT_EXPANDED_MAX_ITEMS > CHAT_RETAINED_MAX_ITEMS);
}
