#[test]
fn refresh_decision_handles_expiry_and_duplicate_unauthorized_responses() {
    let credentials = OAuthCredentials {
        id_token: None,
        chatgpt_account_id: None,
        access_token: "current-access".into(),
        refresh_token: "refresh".into(),
        expires_at: 1_000,
    };

    assert!(!refresh_required(&credentials, 600, None));
    assert!(refresh_required(&credentials, 700, None));
    assert!(refresh_required(
        &credentials,
        600,
        Some("current-access")
    ));
    assert!(!refresh_required(&credentials, 600, Some("older-access")));
}
