use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider-specific configuration.
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfig {
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub organization: Option<String>,
}

impl std::fmt::Debug for ProviderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderConfig")
            .field("api_key", &self.api_key.as_ref().map(|_| "<REDACTED>"))
            .field("api_key_len", &self.api_key.as_ref().map(|k| k.len()))
            .field("base_url", &self.base_url)
            .field("organization", &self.organization)
            .field("headers_count", &self.headers.len())
            .finish()
    }
}
