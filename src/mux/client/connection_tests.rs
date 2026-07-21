use super::supported_version;

#[test]
fn current_client_accepts_the_rollover_protocol_window() {
    assert!(!supported_version(5));
    assert!(supported_version(6));
    assert!(supported_version(7));
    assert!(!supported_version(8));
}
