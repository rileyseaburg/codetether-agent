//! Worker registration models projection.
use codetether_provider_core::ModelInfo;
use std::collections::HashMap;

pub fn build_models_array(models: &HashMap<String, Vec<ModelInfo>>) -> Vec<serde_json::Value> {
    models
        .iter()
        .flat_map(|(provider, infos)| infos.iter().map(move |info| model_json(provider, info)))
        .collect()
}

fn model_json(provider: &str, info: &ModelInfo) -> serde_json::Value {
    let mut model = serde_json::json!({ "id": format!("{provider}/{}", info.id), "name": &info.id, "provider": provider, "provider_id": provider });
    if let Some(input_cost) = info.input_cost_per_million {
        model["input_cost_per_million"] = serde_json::json!(input_cost);
    }
    if let Some(output_cost) = info.output_cost_per_million {
        model["output_cost_per_million"] = serde_json::json!(output_cost);
    }
    model
}
