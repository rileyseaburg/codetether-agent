//! Provider availability checks and swarm-specific fallback selection.
use super::model_defaults::default_model_for_provider;
use anyhow::{Result, anyhow};
use codetether_provider_core::routing::parse_model_string;
use codetether_provider_policy::choose_default_provider;

pub(super) fn resolve_requested(model_ref: &str, providers: &[&str]) -> Result<(String, String)> {
    let (provider, model) = parse_model_string(model_ref);
    let provider = provider.map(|name| if name == "zhipuai" { "zai" } else { name });
    let provider = match provider {
        Some(name) if providers.contains(&name) => name.to_string(),
        Some(name) => anyhow::bail!(
            "Provider '{name}' selected explicitly but is unavailable. Available providers: {}",
            providers.join(", ")
        ),
        None => default_provider(providers)?,
    };
    let model = if model.trim().is_empty() {
        default_model_for_provider(&provider)
    } else {
        model.to_string()
    };
    Ok((provider, model))
}

pub(super) fn resolve_default(providers: &[&str]) -> Result<(String, String)> {
    let provider = default_provider(providers)?;
    let model = default_model_for_provider(&provider);
    Ok((provider, model))
}

fn default_provider(providers: &[&str]) -> Result<String> {
    choose_default_provider(providers)
        .map(str::to_string)
        .ok_or_else(|| anyhow!("No providers available for swarm execution"))
}
