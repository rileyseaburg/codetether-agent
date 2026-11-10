//! Bedrock InvokeModel adapter for native Anthropic Messages models
//! (e.g. `claude-fable-5`).
//!
//! This module translates the native Anthropic Messages request/response
//! shape. Fable 5.2 supports Converse streaming and bypasses the buffered
//! adapter for streaming calls; older variants retain their existing route.

pub mod body;
pub mod complete;
pub mod error;
pub mod invoke_convert;
pub mod invoke_msgconvert;
pub mod response;
pub(in crate::provider) mod retention;
mod routing;
pub mod stream;

#[cfg(test)]
mod tests;

use crate::provider::BedrockProvider;

impl BedrockProvider {
    /// Whether `model_id` should route through the InvokeModel adapter.
    pub(in crate::provider) fn should_use_invoke_model(model_id: &str) -> bool {
        model_id.to_ascii_lowercase().contains("claude-fable-5")
    }
}
