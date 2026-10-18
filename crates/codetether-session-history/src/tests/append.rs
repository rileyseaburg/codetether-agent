use super::fixture::text;
use crate::History;
use codetether_provider_core::{Message, Role};

#[test]
fn append_then_view_is_monotonic() {
    let mut buf: Vec<Message> = Vec::new();
    let mut history = History::new(&mut buf);

    assert!(history.is_empty());
    assert_eq!(history.len(), 0);

    history.append(text(Role::User, "a"));
    history.append(text(Role::Assistant, "b"));
    history.append(text(Role::Tool, "c"));

    let view = history.view();
    assert_eq!(view.len(), 3);
    assert!(matches!(view[0].role, Role::User));
    assert!(matches!(view[1].role, Role::Assistant));
    assert!(matches!(view[2].role, Role::Tool));
}
