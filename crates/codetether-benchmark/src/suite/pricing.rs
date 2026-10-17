//! Lookup and log token prices once per model.
use crate::pricing::ModelPricing;
use tracing::{info, warn};
pub(super) async fn fetch_pricing(provider_name: &str, model_name: &str) -> Option<ModelPricing> {
    let pricing = crate::http::fetch_pricing(provider_name, model_name).await;
    if let Some(ref p) = pricing {
        info!(
            "Model pricing: ${:.2}/M input, ${:.2}/M output",
            p.input_per_m, p.output_per_m
        );
    } else {
        warn!(
            "Could not fetch pricing for {}:{}, using fallback estimates",
            provider_name, model_name
        );
    }

    pricing
}
