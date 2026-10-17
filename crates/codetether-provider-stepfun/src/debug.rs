//! Redacted diagnostic formatting.
use crate::*;
impl std::fmt::Debug for StepFunProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StepFunProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .field("client", &"<reqwest::Client>")
            .finish()
    }
}
