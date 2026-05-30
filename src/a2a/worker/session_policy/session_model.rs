use anyhow::Result;

use crate::provider::parse_model_string;

pub(super) struct ModelSelection {
    pub provider: String,
    pub model: String,
}

pub(super) fn ensure_providers(providers: &[&str]) -> Result<()> {
    if !providers.is_empty() {
        return Ok(());
    }
    anyhow::bail!(
        "No LLM providers available (0 providers loaded). Configure API keys in HashiCorp Vault or set environment variables. Vault address: {}",
        std::env::var("VAULT_ADDR").unwrap_or_else(|_| "(not set)".into())
    )
}

pub(super) fn select(
    model: Option<&str>,
    providers: &[&str],
    tier: Option<&str>,
) -> Result<ModelSelection> {
    let (provider_name, model_id) = model.map(parse_model).unwrap_or((None, String::new()));
    if let Some(name) = provider_name.as_deref()
        && !providers.contains(&name)
    {
        anyhow::bail!(
            "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
            name,
            providers.join(", ")
        );
    }
    let provider = provider_name
        .as_deref()
        .unwrap_or_else(|| super::super::choose_provider_for_tier(providers, tier))
        .to_string();
    let model = if model_id.is_empty() {
        super::super::default_model_for_provider(&provider, tier)
    } else {
        model_id
    };
    Ok(ModelSelection { provider, model })
}

fn parse_model(model: &str) -> (Option<String>, String) {
    let (provider, model_id) = parse_model_string(model);
    let provider = provider.map(|value| if value == "zhipuai" { "zai" } else { value });
    (provider.map(ToString::to_string), model_id.to_string())
}
