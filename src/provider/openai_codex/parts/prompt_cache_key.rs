impl OpenAiCodexProvider {
    /// Returns a stable, privacy-safe routing key for a workspace prompt.
    fn prompt_cache_key(instructions: &str) -> String {
        use sha2::{Digest, Sha256};

        let digest = Sha256::digest(instructions.as_bytes());
        format!("codetether:{}", hex::encode(&digest[..16]))
    }
}
