//! Coordination identity validation tests.

#[test]
fn empty_and_oversized_identities_are_rejected() {
    assert!(super::super::coordination_identity::valid(" ").is_err());
    assert!(super::super::coordination_identity::valid(&"x".repeat(257)).is_err());
    assert!(super::super::coordination_identity::valid("agent-a").is_ok());
}
