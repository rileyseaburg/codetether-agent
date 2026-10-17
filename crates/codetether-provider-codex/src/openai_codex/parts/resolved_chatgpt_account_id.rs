impl OpenAiCodexProvider {
    fn resolved_chatgpt_account_id(&self, access_token: &str) -> Option<String> {
        self.chatgpt_account_id
            .clone()
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(access_token))
    }
}
