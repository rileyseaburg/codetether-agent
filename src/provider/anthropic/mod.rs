//! Anthropic-compatible provider implementation using the Messages API.
//!
//! This module contains the provider shell and delegates request conversion,
//! model metadata, HTTP execution, and response parsing to focused submodules.
//! The provider is responsible for holding shared configuration used by those
//! submodules, including the HTTP client, credential, endpoint, provider label,
//! and prompt-caching preference.

mod anthropic_models;
mod body;
mod cache;
mod complete;
mod complete_error;
mod complete_http;
mod complete_stream;
mod config;
mod constants;
mod convert;
mod convert_parts;
mod convert_role;
mod convert_tools;
mod debug;
mod error;
mod minimax_credits_models;
mod minimax_highspeed_models;
mod minimax_models;
mod models;
mod parse_content;
mod parse_response;
mod parse_usage;
mod provider_impl;
mod response;
mod sanitize;
mod sse_block_parser;
mod sse_line;
mod sse_message_delta;
mod sse_stream;
mod sse_stream_poll;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
#[path = "tests_sanitize.rs"]
mod tests_sanitize;
mod validation;

use reqwest::Client;

/// Provider for Anthropic Messages API compatible backends.
///
/// `AnthropicProvider` stores the configuration and shared HTTP client needed
/// to send chat completion requests to Anthropic or to another service that
/// implements the same Messages API shape. Request construction, response
/// parsing, validation, and model metadata are implemented in sibling modules
/// so this type remains a compact provider state container.
///
/// The API key is kept as process memory state for authenticated requests and
/// must never be exposed through diagnostic output; the custom debug
/// implementation in the `debug` module redacts it.
pub struct AnthropicProvider {
    /// Reusable HTTP client used for provider requests.
    pub(crate) client: Client,
    /// Secret API credential sent to the configured backend.
    pub(crate) api_key: String,
    /// Base URL for the Anthropic-compatible Messages API endpoint.
    pub(crate) base_url: String,
    /// Human-readable provider identifier used in model routing and errors.
    pub(crate) provider_name: String,
    /// Whether prompt-caching request metadata should be emitted when supported.
    pub(crate) enable_prompt_caching: bool,
}
