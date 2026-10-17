//! Credential-redacting debug output.
use super::ZaiProvider;
impl std::fmt::Debug for ZaiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZaiProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"<REDACTED>")
            .finish()
    }
}
