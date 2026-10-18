//! HTTP request callback parsing regressions.
use crate::parse_oauth_callback_request;

#[test]
fn parses_oauth_callback_http_request() {
    let request =
        "GET /auth/callback?code=abc123&state=xyz789 HTTP/1.1\r\nHost: localhost:1455\r\n\r\n";
    let (code, state) =
        parse_oauth_callback_request(request).expect("expected valid callback request");
    assert_eq!(code, "abc123");
    assert_eq!(state, "xyz789");
}

#[test]
fn parses_post_callback_with_query_params() {
    let request =
        "POST /auth/callback?code=abc123&state=xyz789 HTTP/1.1\r\nHost: localhost:1455\r\n\r\n";
    let (code, state) =
        parse_oauth_callback_request(request).expect("expected POST callback parse");
    assert_eq!(code, "abc123");
    assert_eq!(state, "xyz789");
}

#[test]
fn parses_post_form_encoded_callback_request() {
    let request = "POST /auth/callback HTTP/1.1\r\nHost: localhost:1455\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 25\r\n\r\ncode=abc123&state=xyz789";
    let (code, state) =
        parse_oauth_callback_request(request).expect("expected form POST callback parse");
    assert_eq!(code, "abc123");
    assert_eq!(state, "xyz789");
}

#[test]
fn rejects_unsupported_callback_method() {
    let request = "OPTIONS /auth/callback HTTP/1.1\r\nHost: localhost:1455\r\n\r\n";
    let err =
        parse_oauth_callback_request(request).expect_err("expected unsupported callback method");
    assert!(err.to_string().contains("Unsupported callback method"));
}
