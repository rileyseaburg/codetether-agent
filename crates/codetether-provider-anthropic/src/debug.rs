//! Debug formatting that redacts Anthropic credentials.
//!
//! This module provides the [`std::fmt::Debug`] implementation for
//! [`AnthropicProvider`]. The implementation is intentionally separated from
//! provider request logic so diagnostic formatting can focus on safe visibility
//! of configuration details without exposing secrets.

use super::AnthropicProvider;

/// Formats [`AnthropicProvider`] for diagnostics without leaking the API key.
///
/// The debug output includes non-secret configuration such as the base URL,
/// provider name, prompt-caching setting, and API key length. The API key value
/// itself is always rendered as `"<REDACTED>"`, which allows logs and panic
/// diagnostics to identify provider configuration issues without disclosing
/// credentials.
///
/// # Parameters
///
/// * `f` - Formatter supplied by the standard library debug machinery.
///
/// # Returns
///
/// A [`std::fmt::Result`] indicating whether writing to the formatter
/// succeeded.
///
/// # Side Effects
///
/// Writes the redacted provider representation to the supplied formatter. It
/// does not mutate the provider or perform any network or filesystem I/O.
impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .field("base_url", &self.base_url)
            .field("provider_name", &self.provider_name)
            .field("enable_prompt_caching", &self.enable_prompt_caching)
            .finish()
    }
}
