use super::*;
use codetether_provider_core::routing::parse_model_string;
pub(super) fn select_model(
    config: &SwarmConfig,
    provider_list: &[&str],
) -> Result<(String, String)> {
    // Parse model from config, env var, or use default
    let model_str = config
        .model
        .clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok());

    let selected = if let Some(ref model_str) = model_str {
        let (prov, mod_id) = parse_model_string(model_str);
        let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
        let provider = if let Some(explicit_provider) = prov {
            if provider_list.contains(&explicit_provider) {
                explicit_provider.to_string()
            } else {
                anyhow::bail!(
                    "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
                    explicit_provider,
                    provider_list.join(", ")
                );
            }
        } else {
            choose_default_provider(provider_list)
                .ok_or_else(|| anyhow::anyhow!("No providers available for orchestration"))?
                .to_string()
        };
        let model = if mod_id.trim().is_empty() {
            default_model_for_provider(&provider)
        } else {
            mod_id.to_string()
        };
        (provider, model)
    } else {
        let provider = choose_default_provider(provider_list)
            .ok_or_else(|| anyhow::anyhow!("No providers available for orchestration"))?
            .to_string();
        let model = default_model_for_provider(&provider);
        (provider, model)
    };
    Ok(selected)
}
