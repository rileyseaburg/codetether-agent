impl OpenAiCodexProvider {
    fn using_chatgpt_backend(&self) -> bool {
        self.static_api_key.is_none()
    }
}
