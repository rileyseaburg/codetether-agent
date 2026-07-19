use super::{exhausted_request_retryable, is_retryable_request, terminal_stream_retryable};

fn classified(message: &str) -> bool {
    is_retryable_request(&anyhow::anyhow!(message.to_string()))
}

#[test]
fn formatted_server_status_is_retryable() {
    assert!(classified("OpenAI API error (503 Service Unavailable)"));
}

#[test]
fn rate_limit_is_not_retried_by_request_layer() {
    assert!(!classified("OpenAI API error (429 Too Many Requests)"));
}

#[test]
fn generic_request_id_error_is_not_retryable() {
    assert!(!classified("Unknown model; request ID abc"));
}

#[test]
fn network_failure_is_retryable() {
    assert!(classified("connection reset by peer"));
}

#[test]
fn terminal_status_matches_codex_stream_policy() {
    assert!(terminal_stream_retryable(&anyhow::anyhow!(
        "OpenAI API error (403 Forbidden)"
    )));
    assert!(terminal_stream_retryable(&anyhow::anyhow!(
        "OpenAI API error (401 Unauthorized)"
    )));
    assert!(!terminal_stream_retryable(&anyhow::anyhow!(
        "OpenAI API error (400 Bad Request)"
    )));
}

#[test]
fn websocket_handshake_status_is_tagged_for_stream_recovery() {
    let retryable = super::tag_stream_open(anyhow::anyhow!("HTTP error: 403 Forbidden"));
    let permanent = super::tag_stream_open(anyhow::anyhow!("HTTP error: 429 Too Many Requests"));
    assert!(retryable.to_string().starts_with("codex-retryable:"));
    assert!(permanent.to_string().starts_with("codex-permanent:"));
}

#[test]
fn overload_code_stops_after_request_budget() {
    assert!(!exhausted_request_retryable("503 server_is_overloaded"));
    assert!(exhausted_request_retryable("503 internal server error"));
}
