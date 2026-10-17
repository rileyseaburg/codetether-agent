//! Auto-chat default model for provider.
use super::*;

pub(super) fn default_model_for_provider(provider_name: &str) -> Option<&'static str> {
    let defaults = preferred_models_for_provider(provider_name);
    defaults.first().copied()
}
