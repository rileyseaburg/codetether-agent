//! Worker model select policy.
use super::*;

/// Resolve an explicit model reference or the worker's defaults.
///
/// # Arguments
/// * `model` - Optional provider/model reference.
/// * `providers` - Available provider names.
/// * `tier` - Optional model tier used by default selection.
///
/// # Returns
/// The effective provider name and model ID.
///
/// # Errors
/// Returns an error when an explicitly named provider is unavailable.
///
/// # Panics
/// Panics when default selection is needed and `providers` is empty.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::execution_policy::model_selection::select;
/// let selected = select(Some("openai/gpt-4o"), &["openai"], None).unwrap();
/// assert_eq!(selected.provider, "openai");
/// assert_eq!(selected.model, "gpt-4o");
/// ```
pub fn select(
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
        .unwrap_or_else(|| choose_provider_for_tier(providers, tier))
        .to_string();
    let model = if model_id.is_empty() {
        default_model_for_provider(&provider, tier)
    } else {
        model_id
    };
    Ok(ModelSelection { provider, model })
}
