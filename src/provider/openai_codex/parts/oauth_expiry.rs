impl OpenAiCodexProvider {
    fn oauth_expiry(expires_in: u64) -> Result<u64> {
        Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + expires_in)
    }
}
