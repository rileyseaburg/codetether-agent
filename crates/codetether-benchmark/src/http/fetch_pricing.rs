//! Optional pricing lookup from models.dev.
use crate::pricing::ModelPricing;
/// Fetch per-million input and output token prices.
///
/// # Arguments
/// * `provider_id` - Provider key in the models.dev catalog.
/// * `model_id` - Model key within that provider.
/// # Returns
/// Prices, or None when the request, decoding, or catalog lookup fails.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let price = codetether_benchmark::http::fetch_pricing("openai", "gpt-4o").await;
/// if let Some(price) = price { assert!(price.input_per_m >= 0.0); }
/// # }
/// ```
pub async fn fetch_pricing(provider_id: &str, model_id: &str) -> Option<ModelPricing> {
    let url = "https://models.dev/api.json";
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let resp = client.get(url).send().await.ok()?;
    let data: serde_json::Value = resp.json().await.ok()?;

    // Navigate: providers[provider_id].models[model_id].cost
    let cost = data
        .get(provider_id)?
        .get("models")?
        .get(model_id)?
        .get("cost")?;

    let input = cost.get("input")?.as_f64()?;
    let output = cost.get("output")?.as_f64()?;

    Some(ModelPricing {
        input_per_m: input,
        output_per_m: output,
    })
}
