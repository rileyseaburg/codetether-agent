//! Provider-specific model capability enrichment.

use crate::provider::ModelInfo;

use super::types::ModelCapability;

pub(super) fn capability(model: ModelInfo) -> ModelCapability {
    let provider = model.provider;
    let canonical_id = model.id;
    ModelCapability {
        selectable_id: format!("{provider}/{canonical_id}"),
        aliases: aliases(&provider, &canonical_id),
        qualifiers: qualifiers(&provider, &canonical_id),
        provider,
        canonical_id,
        available: true,
        source: "provider.list_models",
    }
}

fn aliases(provider: &str, model: &str) -> Vec<String> {
    if provider != "openai-codex" {
        return Vec::new();
    }
    crate::provider::openai_codex::service_tier_catalog::suffixes(model)
        .iter()
        .map(|suffix| format!("{provider}/{model}{suffix}"))
        .collect()
}

fn qualifiers(provider: &str, model: &str) -> Vec<String> {
    if provider != "openai-codex" {
        return Vec::new();
    }
    crate::provider::openai_codex::reasoning_catalog::supported_levels(model)
        .iter()
        .map(|level| format!(":{level}"))
        .collect()
}
