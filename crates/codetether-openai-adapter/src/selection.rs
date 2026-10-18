//! Resolve a request model against configured provider names.
use crate::{OpenAiApiResult, normalize_model_reference, openai_bad_request, openai_error};
use axum::http::StatusCode;
/// Select a configured provider and resolve its model options.
/// # Arguments
/// * `model` - Requested model name, optionally provider-qualified.
/// * `providers` - Configured provider names in registry order.
/// * `tier` - Optional service tier.
/// * `effort` - Optional reasoning effort.
/// # Returns
/// The selected provider name and provider-local model identifier.
/// # Errors
/// Returns an OpenAI API error for missing providers, ambiguous selection,
/// invalid model syntax or unsupported options.
/// # Examples
/// ```
/// let (provider, model) = codetether_openai_adapter::select_request_model("model", &["openai"], None, None).unwrap();
/// assert_eq!(provider, "openai");
/// assert_eq!(model, "model");
/// ```
pub fn select_request_model(
    model: &str,
    providers: &[&str],
    tier: Option<&str>,
    effort: Option<&str>,
) -> OpenAiApiResult<(String, String)> {
    if providers.is_empty() {
        return Err(openai_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "No providers are configured in Vault",
            "server_error",
            "no_providers",
        ));
    }

    let normalized_model = normalize_model_reference(model);
    let (maybe_provider, model_id) =
        codetether_provider_core::routing::parse_model_string(&normalized_model);
    let model_id = if maybe_provider.is_some() {
        if model_id.trim().is_empty() {
            return Err(openai_bad_request(
                "Model must be in `provider/model` format",
            ));
        }
        model_id.to_string()
    } else {
        model.trim().to_string()
    };

    let selected_provider = crate::selection_provider::choose(maybe_provider, providers)?;
    let model_id = crate::options::apply(&selected_provider, &model_id, tier, effort)
        .map_err(openai_bad_request)?;
    Ok((selected_provider, model_id))
}
