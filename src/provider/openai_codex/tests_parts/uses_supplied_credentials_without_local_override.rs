#[tokio::test]
async fn uses_supplied_credentials_without_local_override() {
    let credentials = OAuthCredentials {
        id_token: Some("vault-id".into()),
        chatgpt_account_id: Some("vault-account".into()),
        access_token: "vault-access".into(),
        refresh_token: "vault-refresh".into(),
        expires_at: u64::MAX,
    };

    let provider = OpenAiCodexProvider::from_credentials(credentials);
    assert!(provider.credential_store.is_none());
    let credentials = provider
        .stored_credentials
        .expect("supplied credentials should be retained");
    let stored = credentials.read().await;

    assert_eq!(stored.credentials.access_token, "vault-access");
    assert_eq!(stored.credentials.refresh_token, "vault-refresh");
    assert_eq!(stored.credentials.id_token.as_deref(), Some("vault-id"));
    assert_eq!(provider.chatgpt_account_id.as_deref(), Some("vault-account"));
}

#[test]
fn vault_credentials_enable_durable_refresh_storage() {
    let credentials = OAuthCredentials {
        id_token: None,
        chatgpt_account_id: Some("account".into()),
        access_token: "access".into(),
        refresh_token: "refresh".into(),
        expires_at: 123,
    };

    let provider = OpenAiCodexProvider::from_vault_credentials("codex", credentials);
    let store = provider
        .credential_store
        .expect("Vault credentials should retain their store");

    assert_eq!(store.provider_id, "codex");
}
