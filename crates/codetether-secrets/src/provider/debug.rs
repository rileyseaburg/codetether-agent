//! Redacted debug output for provider credentials.

use crate::ProviderSecrets;

impl std::fmt::Debug for ProviderSecrets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderSecrets")
            .field("api_key", &self.api_key.as_ref().map(|_| "<REDACTED>"))
            .field("api_key_len", &self.api_key.as_ref().map(|k| k.len()))
            .field("base_url", &self.base_url)
            .field("organization", &self.organization)
            .field("headers_present", &self.headers.is_some())
            .field("extra_fields", &self.extra.len())
            .finish()
    }
}
