//! Build the native Anthropic Messages request body for InvokeModel.

use crate::provider::CompletionRequest;
use crate::provider::bedrock::invoke::{invoke_convert, invoke_msgconvert};
use serde_json::{Value, json};

/// Translate a [`CompletionRequest`] into a native Anthropic Messages body.
pub(in crate::provider::bedrock) fn build_anthropic_messages_body(
    request: &CompletionRequest,
    model_id: &str,
) -> Value {
    use crate::provider::bedrock::{body::fields, convert, output_budget};
    let (system_parts, converse_messages) = convert::convert_messages(&request.messages);
    let messages = invoke_msgconvert::remap_messages_native(&converse_messages);
    let mut body = json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": output_budget::effective_max_tokens(request.max_tokens, model_id),
        "messages": messages,
    });

    if !system_parts.is_empty() {
        let system: Vec<Value> = system_parts
            .iter()
            .map(|p| json!({"type": "text", "text": p.get("text").cloned().unwrap_or(json!(""))}))
            .collect();
        body["system"] = Value::Array(system);
    }
    if let Some(top_p) = request.top_p {
        body["top_p"] = json!(top_p);
    }
    if let Some(temp) = request.temperature
        && !output_budget::has_encrypted_reasoning(model_id)
    {
        body["temperature"] = json!(temp);
    }
    if !request.stop.is_empty() {
        body["stop_sequences"] = json!(request.stop);
    }

    let tools = invoke_convert::convert_tools_native(&request.tools);
    if !tools.is_empty() {
        body["tools"] = Value::Array(tools);
    }

    if let Some(fields) = fields::additional_model_request_fields(model_id)
        && let Some(map) = fields.as_object()
    {
        for (key, value) in map {
            // `output_config` / adaptive `thinking` are Converse-only
            // extensions; the native Messages InvokeModel body rejects them.
            if key == "output_config" || key == "thinking" {
                continue;
            }
            body[key] = value.clone();
        }
    }
    body
}
