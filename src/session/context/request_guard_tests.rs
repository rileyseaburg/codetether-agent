//! Regression checks for final input budgets and stable cached prefixes.
use super::finish;
use crate::provider::Role;
#[path = "request_guard_fixtures.rs"]
mod fixtures;
use fixtures::{request, text};

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

#[test]
fn oversized_optional_context_is_omitted_not_sent() {
    let result = finish(
        request(),
        vec![text(Role::Assistant, &"large ".repeat(100_000))],
    )
    .unwrap();
    assert_eq!(result.messages.len(), 4);
}

#[test]
fn expanded_rlm_or_pinned_state_enters_context_recovery() {
    let mut request = request();
    request
        .messages
        .push(text(Role::Assistant, &"expanded summary ".repeat(100_000)));
    let error = finish(request, vec![]).unwrap_err();
    assert!(crate::session::helper::error::is_prompt_too_long_error(
        &error
    ));
}

#[test]
fn actual_request_output_limit_is_reserved() {
    let mut request = request();
    request.max_tokens = Some(usize::MAX);
    assert!(finish(request, vec![]).is_err());
}
