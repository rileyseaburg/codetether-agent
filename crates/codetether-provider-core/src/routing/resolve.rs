//! Provider alias resolution against the caller-owned registry map.
use super::*;

/// Resolve a model string to a provider and model name.
///
/// Accepts:
/// - `"provider/model"` (e.g. `"openai/gpt-4o"`)
/// - `"model"` alone (uses first available provider)
///
/// # Examples
///
/// ```
/// let providers = std::collections::HashMap::new();
/// assert!(codetether_provider_core::routing::resolve_model(&providers, "gpt-4o").is_err());
/// ```
///
/// # Arguments
/// * `providers` - Existing registry map; it is not reordered or rebuilt.
/// * `model_str` - Bare or provider-qualified model identifier.
/// # Returns
/// A cloned provider handle and the selected model string.
/// # Errors
/// Returns an error for unknown providers or an empty default registry.
pub fn resolve_model(
    providers: &HashMap<String, Arc<dyn Provider>>,
    model_str: &str,
) -> Result<(Arc<dyn Provider>, String)> {
    let (provider_name, model) = parse_model_string(model_str);

    if let Some(provider_name) = provider_name {
        let normalized = match provider_name {
            "local-cuda" | "localcuda" => "local_cuda",
            "zhipuai" => "zai",
            "gsai" | "usai-gov" => "usai",
            other => other,
        };

        let provider = providers.get(normalized).cloned().ok_or_else(|| {
            anyhow::anyhow!(
                "Provider '{}' not found. Available: {:?}",
                normalized,
                providers.keys().map(|s| s.as_str()).collect::<Vec<_>>()
            )
        })?;
        Ok((provider, model.to_string()))
    } else {
        let first_provider = providers
            .values()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No providers available in registry"))?;
        Ok((first_provider.clone(), model_str.to_string()))
    }
}
