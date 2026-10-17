//! OpenAI-compatible provider transport with shared protocol types.
//!
//! Construct `OpenAIProvider` for OpenAI or an API-compatible endpoint and use
//! it through `codetether_provider_core::Provider`.

mod openai;
pub use openai::{OpenAIProvider, alias};

#[cfg(test)]
mod network_tests;
