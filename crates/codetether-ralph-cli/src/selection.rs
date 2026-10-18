//! Ralph model default and provider fallback ordering.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Select an already initialized provider for the Ralph command.
/// # Arguments
/// * `model_arg` - Optional qualified or unqualified model name.
/// * `lookup` - Caller-owned provider lookup.
/// # Returns
/// Provider instance and provider-local model name.
/// # Errors
/// Returns the original missing-provider errors.
/// # Examples
/// ```no_run
/// let _ = codetether_ralph_cli::select_provider(None, &|_| None);
/// ```
pub fn select_provider(
    model_arg: Option<String>,
    lookup: &dyn Fn(&str) -> Option<Arc<dyn Provider>>,
) -> anyhow::Result<(Arc<dyn Provider>, String)> {
    // Parse provider/model from --model flag (e.g. "zai/glm-5" → provider=zai, model=glm-5)
    let model_ref = model_arg.unwrap_or_else(|| "glm-5".to_string());
    let (provider, model) = if let Some((prov_name, model_name)) = model_ref.split_once('/') {
        let prov = lookup(prov_name)
            .ok_or_else(|| anyhow::anyhow!("Provider '{prov_name}' not found in Vault"))?;
        (prov, model_name.to_string())
    } else {
        let prov = lookup("zai")
            .or_else(|| lookup("moonshotai"))
            .or_else(|| lookup("openai"))
            .ok_or_else(|| anyhow::anyhow!("No provider available in Vault"))?;
        (prov, model_ref)
    };

    Ok((provider, model))
}
