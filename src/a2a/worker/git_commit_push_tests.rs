//! Session network policy gate for automatic A2A pushes.

#[test]
fn push_requires_session_network_authority() {
    assert!(super::require_network(false).is_err());
    assert!(super::require_network(true).is_ok());
}