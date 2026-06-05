//! Construction and environment configuration for Anthropic providers.
//!
//! This module defines the constructors for [`AnthropicProvider`] and the small
//! environment-driven configuration helpers used during provider creation. It
//! is responsible for selecting the public Anthropic API defaults, accepting
//! custom Anthropic-compatible endpoints, and deciding whether prompt caching
//! should be enabled for the provider instance.
//!
//! Prompt caching is controlled by `CODETETHER_ANTHROPIC_PROMPT_CACHING` when
//! that environment variable is present. If it is absent or cannot be parsed,
//! provider-specific defaults are used: MiniMax Anthropic-compatible providers
//! enable prompt caching by default, while the standard Anthropic provider does
//! not opt in here.

use anyhow::Result;

use super::AnthropicProvider;
use super::constants::ANTHROPIC_API_BASE;

impl AnthropicProvider {
    /// Create a provider that targets the public Anthropic API.
    ///
    /// This constructor uses the default Anthropic Messages API base URL and
    /// registers the provider name as `"anthropic"`. The API key is stored on
    /// the provider for later request authentication; validation that the key
    /// is non-empty happens before provider operations are executed.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Anthropic API key used for `x-api-key` request
    ///   authentication.
    ///
    /// # Returns
    ///
    /// Returns a configured [`AnthropicProvider`] using the shared HTTP client
    /// and the public Anthropic API base URL.
    ///
    /// # Errors
    ///
    /// Currently returns the same result as [`AnthropicProvider::with_base_url`].
    /// The constructor is fallible to preserve a consistent provider
    /// initialization contract.
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_base_url(api_key, ANTHROPIC_API_BASE.to_string(), "anthropic")
    }

    /// Create a provider with a custom Anthropic-compatible base URL.
    ///
    /// This constructor is used for both the public Anthropic provider and
    /// Anthropic-compatible backends such as MiniMax. It records the provider
    /// name, endpoint, API key, and prompt-caching policy, then reuses the
    /// crate-wide shared HTTP client for outbound requests.
    ///
    /// The prompt-caching setting is resolved during construction. The
    /// `CODETETHER_ANTHROPIC_PROMPT_CACHING` environment variable takes
    /// precedence when it contains a recognized boolean value. Otherwise,
    /// provider-specific defaults are applied.
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key sent with future Messages API requests.
    /// * `base_url` - Base URL for the Anthropic-compatible API, without
    ///   requiring the `/v1/messages` suffix.
    /// * `provider_name` - Logical provider name used for routing, diagnostics,
    ///   model metadata, and provider-specific defaults.
    ///
    /// # Returns
    ///
    /// Returns a configured [`AnthropicProvider`] ready to issue completion
    /// requests once an operation validates the API key.
    ///
    /// # Errors
    ///
    /// This constructor currently does not perform fallible work beyond the
    /// shared provider initialization contract, but it returns [`Result`] so
    /// callers can treat all provider constructors uniformly.
    ///
    /// # Side Effects
    ///
    /// Reads `CODETETHER_ANTHROPIC_PROMPT_CACHING` from the process
    /// environment and emits a debug tracing event containing non-secret
    /// provider configuration metadata.
    pub fn with_base_url(
        api_key: String,
        base_url: String,
        provider_name: impl Into<String>,
    ) -> Result<Self> {
        let provider_name = provider_name.into();
        let enable_prompt_caching = prompt_caching_enabled(&provider_name);
        tracing::debug!(
            provider = %provider_name,
            api_key_len = api_key.len(),
            base_url = %base_url,
            enable_prompt_caching,
            "Creating Anthropic provider"
        );
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            api_key,
            base_url,
            provider_name,
            enable_prompt_caching,
        })
    }
}

/// Resolve whether prompt caching should be enabled for a provider instance.
///
/// The environment variable `CODETETHER_ANTHROPIC_PROMPT_CACHING` overrides
/// provider defaults when it contains a recognized boolean value. Unset or
/// unrecognized values fall back to [`default_prompt_caching`].
///
/// # Arguments
///
/// * `provider_name` - Logical provider name used for default prompt-caching
///   behavior.
///
/// # Returns
///
/// Returns `true` when prompt caching should be enabled for constructed
/// requests, otherwise `false`.
///
/// # Side Effects
///
/// Reads `CODETETHER_ANTHROPIC_PROMPT_CACHING` from the process environment.
fn prompt_caching_enabled(provider_name: &str) -> bool {
    std::env::var("CODETETHER_ANTHROPIC_PROMPT_CACHING")
        .ok()
        .and_then(|v| parse_bool_env(&v))
        .unwrap_or_else(|| default_prompt_caching(provider_name))
}

/// Return the provider-specific prompt-caching default.
///
/// MiniMax Anthropic-compatible providers default to prompt caching because
/// those backends benefit from cache annotations in the request conversion
/// path. Other provider names default to `false` unless overridden by the
/// environment.
///
/// # Arguments
///
/// * `provider_name` - Logical provider name to compare case-insensitively.
///
/// # Returns
///
/// Returns `true` for `"minimax"` and `"minimax-credits"`; returns `false` for
/// all other names.
fn default_prompt_caching(provider_name: &str) -> bool {
    provider_name.eq_ignore_ascii_case("minimax")
        || provider_name.eq_ignore_ascii_case("minimax-credits")
}

/// Parse a permissive boolean value from an environment variable string.
///
/// This helper accepts common truthy and falsy spellings used in shell
/// configuration. Whitespace is ignored and matching is case-insensitive.
/// Unknown values return `None` so callers can distinguish an explicit setting
/// from an invalid or absent one and apply their own fallback behavior.
///
/// # Arguments
///
/// * `value` - Raw environment variable value to parse.
///
/// # Returns
///
/// Returns `Some(true)` for `1`, `true`, `yes`, `on`, or `enabled`; returns
/// `Some(false)` for `0`, `false`, `no`, `off`, or `disabled`; returns `None`
/// for all other values.
pub(crate) fn parse_bool_env(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "enabled" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" => Some(false),
        _ => None,
    }
}
