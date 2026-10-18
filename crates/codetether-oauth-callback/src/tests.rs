//! OAuth callback parsing regression tests.
use crate::{extract_oauth_code_and_state, parse_oauth_callback_request};

#[test]
fn extracts_code_and_state_from_full_callback_url() {
    let input = "http://localhost:1455/auth/callback?code=abc123&state=xyz789";
    let (code, state) = extract_oauth_code_and_state(input).expect("expected callback parse");
    assert_eq!(code, "abc123");
    assert_eq!(state, "xyz789");
}

#[test]
fn extracts_code_and_state_from_raw_query_string() {
    let input = "code=abc123&state=xyz789";
    let (code, state) = extract_oauth_code_and_state(input).expect("expected callback parse");
    assert_eq!(code, "abc123");
    assert_eq!(state, "xyz789");
}

#[test]
fn returns_error_when_state_is_missing() {
    let input = "http://localhost:1455/auth/callback?code=abc123";
    let err = extract_oauth_code_and_state(input).expect_err("expected missing state");
    assert!(err.to_string().contains("OAuth state"));
}
