impl OpenAiCodexProvider {
    fn extract_chatgpt_account_id_from_jwt(jwt: &str) -> Option<String> {
        codetether_codex_auth::CodexOAuth::extract_chatgpt_account_id_from_jwt(jwt)
    }
}
