//! Model catalog selection for Anthropic-compatible providers.
//!
//! This module converts compact, static model catalog entries into public
//! [`ModelInfo`] values exposed by an [`AnthropicProvider`]. It chooses the
//! correct catalog for Anthropic, MiniMax, or MiniMax credits backends based on
//! the provider name stored in the provider configuration.

use crate::provider::ModelInfo;

use super::{AnthropicProvider, anthropic_models, minimax_credits_models, minimax_models};

/// Compact metadata used to build public model information.
///
/// Each tuple field maps directly to the corresponding [`ModelInfo`] field:
/// model identifier, display name, context window, maximum output tokens,
/// vision support, input cost per million tokens, and output cost per million
/// tokens. Tool use and streaming support are fixed to `true` for every model
/// produced by this catalog.
#[derive(Clone, Copy)]
pub(crate) struct Spec(
    /// Provider-facing model identifier used in API requests.
    pub &'static str,
    /// Human-readable model name shown in model listings.
    pub &'static str,
    /// Maximum context window size in tokens.
    pub usize,
    /// Maximum number of output tokens the model can produce.
    pub usize,
    /// Whether the model accepts image content.
    pub bool,
    /// Input-token cost per one million tokens.
    pub f64,
    /// Output-token cost per one million tokens.
    pub f64,
);

impl AnthropicProvider {
    /// Return static model metadata for this Anthropic-compatible provider.
    ///
    /// The catalog is selected from [`self.provider_name`](Self::provider_name):
    /// `minimax-credits` uses the credits catalog, `minimax` uses the standard
    /// MiniMax catalog, and all other provider names use the Anthropic catalog.
    ///
    /// # Returns
    ///
    /// A vector of [`ModelInfo`] entries with provider name, token limits,
    /// capability flags, and pricing populated from the selected static
    /// catalog.
    pub(crate) fn available_models(&self) -> Vec<ModelInfo> {
        for_provider(&self.provider_name)
    }
}

/// Select the model catalog that corresponds to a provider name.
///
/// Provider name matching is ASCII case-insensitive. Unknown provider names are
/// treated as Anthropic-compatible and receive the default Anthropic catalog.
///
/// # Parameters
///
/// * `provider` - Provider identifier from [`AnthropicProvider::provider_name`].
///
/// # Returns
///
/// Public model metadata built from the matching static catalog.
fn for_provider(provider: &str) -> Vec<ModelInfo> {
    if provider.eq_ignore_ascii_case("minimax-credits") {
        return models(provider, minimax_credits_models::MODELS);
    }
    if provider.eq_ignore_ascii_case("minimax") {
        return models(provider, minimax_models::MODELS);
    }
    models(provider, anthropic_models::MODELS)
}

/// Convert a static catalog of compact specs into public model metadata.
///
/// # Parameters
///
/// * `provider` - Provider name to attach to every returned model.
/// * `specs` - Static model specifications for the selected catalog.
///
/// # Returns
///
/// A vector containing one [`ModelInfo`] entry for each supplied [`Spec`].
fn models(provider: &str, specs: &[Spec]) -> Vec<ModelInfo> {
    specs.iter().copied().map(|s| model(provider, s)).collect()
}

/// Convert a compact model specification into a [`ModelInfo`] record.
///
/// The resulting record enables tool use and streaming for all catalog entries,
/// because every Anthropic-compatible backend represented by these catalogs is
/// expected to support those capabilities.
///
/// # Parameters
///
/// * `provider` - Provider name to store in the returned metadata.
/// * `spec` - Compact model specification to expand.
///
/// # Returns
///
/// A fully populated [`ModelInfo`] value for one model.
fn model(provider: &str, spec: Spec) -> ModelInfo {
    ModelInfo {
        id: spec.0.to_string(),
        name: spec.1.to_string(),
        provider: provider.to_string(),
        context_window: spec.2,
        max_output_tokens: Some(spec.3),
        supports_vision: spec.4,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(spec.5),
        output_cost_per_million: Some(spec.6),
    }
}
