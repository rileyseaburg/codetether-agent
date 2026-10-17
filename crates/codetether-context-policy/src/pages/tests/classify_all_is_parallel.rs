//! Existing page policy regression.
use super::*;

#[test]
fn classify_all_is_parallel() {
    let msgs = vec![user("a"), assistant("b"), tool_result("c", "out")];
    let pages = classify_all(&msgs);
    assert_eq!(pages.len(), 3);
    assert_eq!(pages[0], PageKind::Conversation);
    assert_eq!(pages[1], PageKind::Conversation);
    assert_eq!(pages[2], PageKind::Evidence);
}
