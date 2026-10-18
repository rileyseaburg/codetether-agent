use super::fixtures::{request, text};
use crate::finish;
use codetether_provider_core::Role;

#[test]
fn oversized_optional_context_is_omitted_not_sent() {
    let result = finish(
        request(),
        vec![text(Role::Assistant, &"large ".repeat(100_000))],
    )
    .unwrap();
    assert_eq!(result.messages.len(), 4);
}
