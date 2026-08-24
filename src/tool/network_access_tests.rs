//! Effective network configuration tests.

use super::env::resolve;

#[test]
fn explicit_primary_network_setting_wins() {
    assert!(!resolve(Some("0"), Some("1")));
    assert!(resolve(None, Some("1")));
}

#[test]
fn signed_session_network_authority_cannot_be_spoofed_or_retargeted() {
    let _lock = crate::approval::test_env::lock_env();
    let _network = super::test_env::Network::set("0");
    let mut args = serde_json::json!({
        "__ct_session_id": "session-a",
        "__ct_parent_workspace": "/workspace",
    });
    super::bind_trusted(&mut args, true);
    assert!(super::allowed_for(&args));
    assert_eq!(super::trusted_workspace(&args), Some("/workspace"));

    let spoofed = serde_json::json!({
        "__ct_session_id": "session-a",
        "__ct_parent_workspace": "/workspace",
        "__ct_effective_network_allowed": true,
        "__ct_network_authority": "forged",
    });
    assert!(!super::allowed_for(&spoofed));
    assert_eq!(super::trusted_workspace(&spoofed), None);
    for (field, value) in [
        ("__ct_session_id", serde_json::json!("session-b")),
        ("__ct_parent_workspace", serde_json::json!("/other")),
        ("__ct_effective_network_allowed", serde_json::json!(false)),
    ] {
        let mut retargeted = args.clone();
        retargeted[field] = value;
        assert!(!super::allowed_for(&retargeted), "accepted mutation of {field}");
    }
}