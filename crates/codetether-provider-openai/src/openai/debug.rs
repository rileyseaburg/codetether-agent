//! Redacted debug representation of provider state.

use super::*;

impl std::fmt::Debug for OpenAIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIProvider")
            .field("provider_name", &self.provider_name)
            .field("api_base", &self.api_base)
            .field("client", &"<async_openai::Client>")
            .finish()
    }
}
