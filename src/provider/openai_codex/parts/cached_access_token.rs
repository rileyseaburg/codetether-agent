impl OpenAiCodexProvider {
    async fn cached_access_token(&self) -> Option<String> {
        let cache = self.cached_tokens.read().await;
        let tokens = cache.as_ref()?;
        (tokens
            .expires_at
            .duration_since(std::time::Instant::now())
            .as_secs()
            > 300)
            .then(|| tokens.access_token.clone())
    }
}
