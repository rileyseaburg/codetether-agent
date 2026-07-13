//! Rendering for provider/model capabilities.

use super::types::ProviderCapability;

pub(super) fn json(capabilities: &[ProviderCapability]) -> anyhow::Result<String> {
    Ok(serde_json::to_string_pretty(capabilities)?)
}

pub(super) fn text(capabilities: &[ProviderCapability]) -> String {
    let mut lines = Vec::new();
    for provider in capabilities {
        lines.push(format!(
            "{} [{}] source={}",
            provider.provider,
            if provider.available {
                "available"
            } else {
                "unavailable"
            },
            provider.source
        ));
        if let Some(error) = &provider.error {
            lines.push(format!("  error: {error}"));
        }
        for model in &provider.models {
            lines.push(format!("  {}", model.selectable_id));
            lines.push(format!("    canonical: {}", model.canonical_id));
            if !model.aliases.is_empty() {
                lines.push(format!("    aliases: {}", model.aliases.join(", ")));
            }
            if !model.qualifiers.is_empty() {
                lines.push(format!("    qualifiers: {}", model.qualifiers.join(", ")));
            }
            lines.push(format!("    source: {}", model.source));
        }
    }
    let providers = capabilities.len();
    let models: usize = capabilities.iter().map(|item| item.models.len()).sum();
    lines.push(format!(
        "{models} models from {providers} configured providers"
    ));
    lines.join("\n")
}
