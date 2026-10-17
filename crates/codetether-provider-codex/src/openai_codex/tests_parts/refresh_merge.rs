#[test]
fn refresh_merge_preserves_identity_and_accepts_rotated_tokens() {
    let previous = OAuthCredentials {
        id_token: Some("identity".into()),
        chatgpt_account_id: Some("account".into()),
        access_token: "old-access".into(),
        refresh_token: "old-refresh".into(),
        expires_at: 100,
    };
    let refreshed = OAuthCredentials {
        id_token: None,
        chatgpt_account_id: None,
        access_token: "new-access".into(),
        refresh_token: "new-refresh".into(),
        expires_at: 200,
    };

    let merged = merge_refreshed_credentials(&previous, refreshed);

    assert_eq!(merged.access_token, "new-access");
    assert_eq!(merged.refresh_token, "new-refresh");
    assert_eq!(merged.id_token.as_deref(), Some("identity"));
    assert_eq!(merged.chatgpt_account_id.as_deref(), Some("account"));
}
