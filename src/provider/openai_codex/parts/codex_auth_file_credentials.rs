impl OpenAiCodexProvider {
    fn codex_auth_file_credentials() -> Option<OAuthCredentials> {
        #[derive(Deserialize)]
        struct AuthFile {
            tokens: AuthTokens,
        }
        #[derive(Deserialize)]
        struct AuthTokens {
            id_token: String,
            access_token: String,
            refresh_token: String,
            account_id: String,
        }
        let home = std::env::var_os("HOME").map(PathBuf::from)?;
        let raw = std::fs::read_to_string(home.join(".codex/auth.json")).ok()?;
        let auth: AuthFile = serde_json::from_str(&raw).ok()?;
        Some(OAuthCredentials {
            id_token: Some(auth.tokens.id_token),
            chatgpt_account_id: Some(auth.tokens.account_id),
            access_token: auth.tokens.access_token,
            refresh_token: auth.tokens.refresh_token,
            expires_at: u64::MAX,
        })
    }
}
