//! Shared endpoint and timeout configuration.
use std::time::Duration;

pub(super) const REQUEST_TIMEOUT: Duration = Duration::from_secs(900);

// Avatar video gen can be slow

pub(super) fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}
