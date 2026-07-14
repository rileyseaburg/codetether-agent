use super::is_retryable;

#[test]
fn openai_request_id_error_is_retryable() {
    assert!(is_retryable(
        "An error occurred while processing your request. You can retry your request. Please include the request ID abc"
    ));
}

#[test]
fn request_id_alone_is_not_retryable() {
    assert!(!is_retryable("Unknown model; request ID abc"));
}

#[test]
fn permanent_status_wins() {
    assert!(!is_retryable(
        "403 forbidden; you can retry; request ID abc"
    ));
}

#[test]
fn formatted_server_status_is_retryable() {
    assert!(is_retryable("OpenAI API error (503 Service Unavailable)"));
}
