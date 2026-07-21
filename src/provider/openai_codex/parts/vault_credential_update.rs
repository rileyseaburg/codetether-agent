fn updated_vault_secret(
    mut secret: crate::secrets::ProviderSecrets,
    credentials: &OAuthCredentials,
) -> crate::secrets::ProviderSecrets {
    secret
        .extra
        .insert("access_token".into(), json!(credentials.access_token));
    secret
        .extra
        .insert("refresh_token".into(), json!(credentials.refresh_token));
    secret
        .extra
        .insert("expires_at".into(), json!(credentials.expires_at));
    if let Some(value) = &credentials.id_token {
        secret.extra.insert("id_token".into(), json!(value));
    }
    if let Some(value) = &credentials.chatgpt_account_id {
        secret.organization = Some(value.clone());
        secret
            .extra
            .insert("chatgpt_account_id".into(), json!(value));
    }
    secret
}
