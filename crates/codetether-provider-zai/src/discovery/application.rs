//! Application discovery URL selection and result reporting.
use crate::ModelInfo;
use reqwest::Client;
use serde_json::Value;
/// Discover models using the application's existing response conventions.
///
/// # Arguments
/// * client - Shared HTTP client.
/// * api_key - Bearer credential used by the discovery request.
/// * base_url - Provider URL; coding endpoints are mapped to the ordinary API.
///
/// # Returns
/// Parsed models, or an empty list when discovery fails.
///
/// # Examples
///
///     # async fn example() {
///     let client = reqwest::Client::new();
///     let models = codetether_provider_zai::discovery::discover_application(
///         &client, "", "http://localhost:1234/api",
///     ).await;
///     let _ = models;
///     # }
pub async fn discover_application(
    client: &Client,
    api_key: &str,
    base_url: &str,
) -> Vec<ModelInfo> {
    let discovery_url = if base_url.contains("/coding/") {
        base_url.replace("/coding/", "/")
    } else {
        base_url.to_owned()
    };
    let url = format!("{discovery_url}/models");
    let Some(payload) = super::fetch::fetch(client, api_key, &url).await else {
        return Vec::new();
    };
    let models = payload
        .get("data")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(super::application_model::decode)
        .collect::<Vec<_>>();
    if models.is_empty() {
        tracing::debug!(url = %url, "Z.AI /models returned no model ids");
    } else {
        tracing::info!(count = models.len(), "Z.AI /models discovery succeeded");
    }
    models
}
