//! Redacted provider diagnostics.
use crate::*;
impl std::fmt::Debug for Glm5Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glm5Provider")
            .field("base_url", &self.base_url)
            .field("model_name", &self.model_name)
            .field("api_key", &"<REDACTED>")
            .finish()
    }
}
