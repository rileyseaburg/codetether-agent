impl OpenAiCodexProvider {
    fn resolved_chatgpt_account_id(&self, access_token: &str) -> Option<String> {
        self.chatgpt_account_id
            .clone()
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(access_token))
            .or_else(|| {
                self.stored_credentials.as_ref().and_then(|creds| {
                    creds
                        .id_token
                        .as_deref()
                        .and_then(Self::extract_chatgpt_account_id_from_jwt)
                        .or_else(|| creds.chatgpt_account_id.clone())
                })
            })
    }
}
