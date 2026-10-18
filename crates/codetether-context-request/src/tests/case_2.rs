use super::fixtures::{request, text};
use crate::finish;
use codetether_provider_core::Role;

#[test]
fn expanded_rlm_or_pinned_state_enters_context_recovery() {
    let mut request = request();
    request
        .messages
        .push(text(Role::Assistant, &"expanded summary ".repeat(100_000)));
    let error = finish(request, vec![]).unwrap_err();
    assert!(codetether_context_policy::request::recovery::is_prompt_too_long_error(&error));
}
