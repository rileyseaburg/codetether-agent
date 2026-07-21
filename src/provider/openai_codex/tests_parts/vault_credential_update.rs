#[test]
fn vault_credential_update_rotates_tokens_and_preserves_metadata() {
    let mut secret = crate::secrets::ProviderSecrets {
        base_url: Some("https://example.test".into()),
        ..Default::default()
    };
    secret.extra.insert("custom".into(), serde_json::json!("kept"));
    let credentials = OAuthCredentials {
        id_token: Some("new-id".into()),
        chatgpt_account_id: Some("new-account".into()),
        access_token: "new-access".into(),
        refresh_token: "new-refresh".into(),
        expires_at: 456,
    };

    let updated = updated_vault_secret(secret, &credentials);

    assert_eq!(updated.base_url.as_deref(), Some("https://example.test"));
    assert_eq!(updated.organization.as_deref(), Some("new-account"));
    assert_eq!(updated.extra["custom"], "kept");
    assert_eq!(updated.extra["access_token"], "new-access");
    assert_eq!(updated.extra["refresh_token"], "new-refresh");
    assert_eq!(updated.extra["id_token"], "new-id");
    assert_eq!(updated.extra["chatgpt_account_id"], "new-account");
    assert_eq!(updated.extra["expires_at"], 456);
}
