//! Provider configuration access and shared protocol convenience methods.
//!
//! [BedrockProvider] exposes its region, shared model aliases, and Converse
//! body builder here; protocol implementations remain in workspace crates.
use super::{BedrockProvider, aliases, body};
use codetether_provider_core::CompletionRequest;
impl BedrockProvider {
    /// Return the configured AWS region.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Resolve a short model alias to a full Bedrock model ID.
    ///
    /// See [`aliases::resolve_model_id`] for details.
    ///
    /// # Arguments
    /// * `model` - Short alias or full Bedrock identifier.
    ///
    /// # Returns
    /// The shared canonical identifier, or the original value for unknown aliases.
    ///
    /// # Examples
    /// ```
    /// use codetether_agent::provider::bedrock::BedrockProvider;
    /// assert_eq!(BedrockProvider::resolve_model_id("custom.model"), "custom.model");
    /// ```
    pub fn resolve_model_id(model: &str) -> &str {
        aliases::resolve_model_id(model)
    }

    /// Build a Converse request body. See [`body::build_converse_body`].
    ///
    /// # Arguments
    /// * `request` - Provider-neutral conversation and controls.
    /// * `model_id` - Resolved Bedrock model identifier.
    ///
    /// # Returns
    /// The Converse JSON body prepared by the protocol crate.
    ///
    /// # Examples
    /// See [body::build_converse_body] for a request-building example.
    pub fn build_converse_body(
        &self,
        request: &CompletionRequest,
        model_id: &str,
    ) -> serde_json::Value {
        body::build_converse_body(request, model_id)
    }
}
