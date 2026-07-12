//! Provider and model resolution for swarm workers.

use crate::provider::parse_model_string;
use crate::swarm::orchestrator::{choose_default_provider, default_model_for_provider};
use anyhow::{Result, anyhow};

#[derive(Clone, Debug, serde::Serialize)]
pub(super) struct ModelSelection {
    pub(super) requested_model: Option<String>,
    pub(super) resolved_provider: String,
    pub(super) resolved_model: String,
}

pub(super) fn resolve(requested: Option<&str>, providers: &[&str]) -> Result<ModelSelection> {
    let (provider, model) = match requested {
        Some(model_ref) => resolve_requested(model_ref, providers)?,
        None => resolve_default(providers)?,
    };
    Ok(ModelSelection {
        requested_model: requested.map(str::to_string),
        resolved_provider: provider,
        resolved_model: model,
    })
}

fn resolve_requested(model_ref: &str, providers: &[&str]) -> Result<(String, String)> {
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

fn resolve_default(providers: &[&str]) -> Result<(String, String)> {
    let provider = default_provider(providers)?;
    let model = default_model_for_provider(&provider);
    Ok((provider, model))
}

fn default_provider(providers: &[&str]) -> Result<String> {
    choose_default_provider(providers)
        .map(str::to_string)
        .ok_or_else(|| anyhow!("No providers available for swarm execution"))
}
