//! Worker registration payload helpers.

use std::collections::HashMap;

use crate::provider::ModelInfo;

pub(super) fn build_models_array(
    models: &HashMap<String, Vec<ModelInfo>>,
) -> Vec<serde_json::Value> {
    models
        .iter()
        .flat_map(|(provider, infos)| infos.iter().map(move |info| model_json(provider, info)))
        .collect()
}

pub(super) fn builtin_agent_defs() -> Vec<serde_json::Value> {
    crate::agent::AgentRegistry::with_builtins()
        .list()
        .iter()
        .map(|info| {
            serde_json::json!({
                "name": info.name, "description": info.description, "mode": format!("{:?}", info.mode).to_lowercase(),
                "native": info.native, "hidden": info.hidden, "model": info.model, "temperature": info.temperature,
                "top_p": info.top_p, "max_steps": info.max_steps,
            })
        })
        .collect()
}

pub(super) fn host_metadata() -> (String, Option<String>) {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let k8s_node_name = std::env::var("K8S_NODE_NAME")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    (hostname, k8s_node_name)
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
