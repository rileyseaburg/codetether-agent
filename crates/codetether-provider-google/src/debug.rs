//! Redacted provider diagnostics.
use crate::*;
impl std::fmt::Debug for GoogleProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .finish()
    }
}
