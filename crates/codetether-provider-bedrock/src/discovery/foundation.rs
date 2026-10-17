//! Foundation model admission and metadata projection.
use super::*;

pub(super) fn foundation_model_to_info(m: &Value) -> Option<ModelInfo> {
    let model_id = m.get("modelId").and_then(|v| v.as_str()).unwrap_or("");
    let model_name = m.get("modelName").and_then(|v| v.as_str()).unwrap_or("");

    let (output_modalities, input_modalities, inference_types) = modalities::read(m);

    if !output_modalities.contains(&"TEXT")
        || (!inference_types.contains(&"ON_DEMAND")
            && !inference_types.contains(&"INFERENCE_PROFILE"))
    {
        return None;
    }

    let name_lower = model_name.to_lowercase();
    if ["rerank", "embed", "safeguard", "sonic", "pegasus"]
        .iter()
        .any(|n| name_lower.contains(n))
    {
        return None;
    }

    let streaming = m
        .get("responseStreamingSupported")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision = input_modalities.contains(&"IMAGE");

    let actual_id = if model_id.starts_with("amazon.") {
        model_id.to_string()
    } else if inference_types.contains(&"INFERENCE_PROFILE") {
        format!("us.{model_id}")
    } else {
        model_id.to_string()
    };

    Some(ModelInfo {
        id: actual_id.clone(),
        name: format!("{model_name} (Bedrock)"),
        provider: "bedrock".to_string(),
        context_window: estimate_context_window(model_id),
        max_output_tokens: Some(estimate_max_output(model_id)),
        supports_vision: vision,
        supports_tools: true,
        supports_streaming: streaming,
        input_cost_per_million: None,
        output_cost_per_million: None,
    })
}
