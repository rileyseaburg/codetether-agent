use super::fixtures::{request, text};
use crate::finish;
use codetether_provider_core::Role;

#[test]
fn optional_context_preserves_history_prefix() {
    let original = request();
    let result = finish(original.clone(), vec![text(Role::System, "fresh recall")]).unwrap();
    assert_eq!(
        serde_json::to_value(&result.messages[..3]).unwrap(),
        serde_json::to_value(&original.messages[..3]).unwrap()
    );
    assert_eq!(result.messages.len(), 5);
    assert_eq!(result.messages[3].role, Role::User);
    assert_eq!(result.messages[4].role, Role::User);
}
