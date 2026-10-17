//! Existing empty_guard regression tests.

use super::check_visible_output;
use codetether_provider_core::ContentPart;

#[test]
fn empty_max_tokens_completion_is_error() {
    assert!(check_visible_output(&[], Some("max_tokens")).is_err());
    let thinking_only = vec![ContentPart::Thinking {
        text: "t".into(),
        signature: None,
    }];
    assert!(check_visible_output(&thinking_only, Some("max_tokens")).is_err());
}

#[test]
fn visible_or_natural_stop_is_ok() {
    let text = vec![ContentPart::Text { text: "hi".into() }];
    assert!(check_visible_output(&text, Some("max_tokens")).is_ok());
    assert!(check_visible_output(&[], Some("end_turn")).is_ok());
}
