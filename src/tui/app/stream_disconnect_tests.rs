use super::is_stream_disconnect;

#[test]
fn classifies_transient() {
    assert!(is_stream_disconnect("premature EOF after 100 bytes"));
    assert!(is_stream_disconnect("broken pipe"));
    assert!(is_stream_disconnect("h2 protocol error: stream closed"));
    assert!(is_stream_disconnect("connection reset by peer"));
    assert!(is_stream_disconnect("unexpected eof"));
    assert!(is_stream_disconnect("network error: transport closed"));
}

#[test]
fn classifies_permanent() {
    assert!(!is_stream_disconnect("401 Unauthorized"));
    assert!(!is_stream_disconnect("403 Forbidden"));
    assert!(!is_stream_disconnect("context length exceeded"));
    assert!(!is_stream_disconnect("invalid api key"));
}
