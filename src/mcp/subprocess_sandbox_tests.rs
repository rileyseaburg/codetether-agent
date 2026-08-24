#[test]
fn policy_preserves_explicit_session_network_authority() {
    let (offline, _) = super::policy(false).expect("offline policy");
    let (online, _) = super::policy(true).expect("online policy");

    assert!(!offline.allow_network);
    assert!(online.allow_network);
}
