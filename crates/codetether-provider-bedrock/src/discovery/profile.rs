//! System inference-profile admission and projection.
use super::*;

pub(super) fn inference_profile_to_info(
    p: &Value,
    models: &HashMap<String, ModelInfo>,
) -> Option<ModelInfo> {
    let pid = p
        .get("inferenceProfileId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let pname = p
        .get("inferenceProfileName")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !pid.starts_with("us.") || models.contains_key(pid) {
        return None;
    }

    if profile_filter::excluded(pname) {
        return None;
    }

    let vision = pid.contains("llama3-2-11b")
        || pid.contains("llama3-2-90b")
        || pid.contains("pixtral")
        || pid.contains("claude-3")
        || pid.contains("claude-sonnet-4")
        || pid.contains("claude-opus-4")
        || pid.contains("claude-haiku-4");

    let display_name = pname.replace("US ", "");
    let display_name = format!("{} (Bedrock)", display_name.trim());

    Some(ModelInfo {
        id: pid.to_string(),
        name: display_name,
        provider: "bedrock".to_string(),
        context_window: estimate_context_window(pid),
        max_output_tokens: Some(estimate_max_output(pid)),
        supports_vision: vision,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    })
}
