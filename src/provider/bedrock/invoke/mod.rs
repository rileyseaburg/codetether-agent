//! Bedrock InvokeModel adapter for native Anthropic Messages models
//! (e.g. `claude-fable-5`).
//!
//! Unlike the Converse API path, these models require a *native* Anthropic
//! Messages request/response schema. This module and its children translate
//! between the crate-internal types and that native shape.

pub mod body;
pub mod complete;
pub mod invoke_convert;
pub mod invoke_msgconvert;
pub mod response;
pub mod stream;

#[cfg(test)]
mod tests;

use crate::provider::bedrock::BedrockProvider;

impl BedrockProvider {
    /// Whether `model_id` should route through the InvokeModel adapter.
    pub(in crate::provider::bedrock) fn should_use_invoke_model(model_id: &str) -> bool {
        model_id.to_ascii_lowercase().contains("claude-fable-5")
    }
}
