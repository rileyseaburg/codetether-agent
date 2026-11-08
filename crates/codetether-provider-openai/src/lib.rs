//! OpenAI-compatible provider transport with shared protocol types.
//!
//! Construct `OpenAIProvider` for OpenAI or an API-compatible endpoint and use
//! it through `codetether_provider_core::Provider`.

mod openai;
pub use openai::{OpenAIProvider, alias};

/// Official Groq OpenAI-compatible inference and model-discovery API root.
pub const GROQ_BASE_URL: &str = "https://api.groq.com/openai/v1";

#[cfg(test)]
mod network_tests;
