use crate::provider::{CompletionResponse, ContentPart, FinishReason, ToolDefinition};
use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;
use uuid::Uuid;

pub fn tool_call_markup_re() -> &'static Regex {
    static TOOL_CALL_RE: OnceLock<Regex> = OnceLock::new();
    TOOL_CALL_RE.get_or_init(|| {
        Regex::new(r"(?s)<tool_call>\s*(?:```(?:json)?\s*)?(\{.*?\})(?:\s*```)?\s*</tool_call>")
            .expect("tool_call regex must compile")
    })
}

pub fn extract_markup_tool_calls(text: &str) -> (String, Vec<(String, String)>) {
    let mut calls = Vec::new();
    let re = tool_call_markup_re();

    for capture in re.captures_iter(text) {
        let Some(block) = capture.get(1).map(|m| m.as_str()) else {
            continue;
        };
        let Ok(payload) = serde_json::from_str::<Value>(block) else {
            continue;
        };
        let Some(name) = payload.get("name").and_then(Value::as_str) else {
            continue;
        };
        let arguments = payload
            .get("arguments")
            .or_else(|| payload.get("args"))
            .or_else(|| payload.get("input"))
            .cloned()
            .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| {
                // When no explicit arguments/args/input key exists,
                // treat all remaining top-level keys (except "name") as the arguments object.
                if let Some(obj) = payload.as_object() {
                    let params: serde_json::Map<String, Value> = obj
                        .iter()
                        .filter(|(k, _)| *k != "name")
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    serde_json::to_string(&Value::Object(params))
                        .unwrap_or_else(|_| "{}".to_string())
                } else {
                    "{}".to_string()
                }
            });
        calls.push((name.to_string(), arguments));
    }

    let cleaned = re.replace_all(text, "").into_owned();
    (cleaned, calls)
}

pub fn normalize_textual_tool_calls(
    mut response: CompletionResponse,
    tools: &[ToolDefinition],
) -> CompletionResponse {
    if response
        .message
        .content
        .iter()
        .any(|p| matches!(p, ContentPart::ToolCall { .. }))
    {
        return response;
    }

    if tools.is_empty() {
        return response;
    }

    let mut rewritten = Vec::with_capacity(response.message.content.len());
    let mut parsed_calls: Vec<(String, String)> = Vec::new();
    let allowed_tools: std::collections::HashSet<&str> =
        tools.iter().map(|t| t.name.as_str()).collect();

    for part in response.message.content {
        match part {
            ContentPart::Text { text } => {
                let (cleaned, calls) = extract_markup_tool_calls(&text);
                for (name, arguments) in calls {
                    if allowed_tools.contains(name.as_str()) {
                        parsed_calls.push((name, arguments));
                    } else {
                        tracing::warn!(tool = %name, "Ignoring unknown <tool_call> tool name");
                    }
                }

                if !cleaned.trim().is_empty() {
                    rewritten.push(ContentPart::Text {
                        text: cleaned.trim().to_string(),
                    });
                }
            }
            other => rewritten.push(other),
        }
    }

    if parsed_calls.is_empty() {
        response.message.content = rewritten;
        return response;
    }

    for (name, arguments) in parsed_calls {
        rewritten.push(ContentPart::ToolCall {
            id: Uuid::new_v4().to_string(),
            name,
            arguments,
            thought_signature: None,
        });
    }

    response.message.content = rewritten;
    response.finish_reason = FinishReason::ToolCalls;
    response
}
