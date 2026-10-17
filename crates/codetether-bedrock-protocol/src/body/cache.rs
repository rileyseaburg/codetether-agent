//! Anthropic prompt-cache breakpoint placement for Converse requests.
//!
//! Cache points mark stable prefix boundaries so repeated agent turns get the
//! input discount:
//!   1. End of `system` — the large, static system prompt.
//!   2. End of `toolConfig.tools` — tool schemas, which rarely change.
//!   3. End of the last message — the whole conversation prefix (sliding).
//!
//! Bedrock allows up to 4 breakpoints per request; three is well under.
//! Disable with `CODETETHER_BEDROCK_PROMPT_CACHE=0`.

use serde_json::{Value, json};

/// Append cache breakpoints when caching applies to `model_id`.
pub(super) fn apply(
    model_id: &str,
    system_parts: &mut Vec<Value>,
    tools: &mut Vec<Value>,
    messages: &mut [Value],
) {
    if !enabled() || !supported(model_id) {
        return;
    }
    if !system_parts.is_empty() {
        system_parts.push(point());
    }
    if !tools.is_empty() {
        tools.push(point());
    }
    if let Some(last) = messages.last_mut()
        && let Some(content) = last.get_mut("content").and_then(Value::as_array_mut)
        && !content.is_empty()
    {
        content.push(point());
    }
}

fn point() -> Value {
    json!({"cachePoint": {"type": "default"}})
}

/// Prompt caching is on by default; `CODETETHER_BEDROCK_PROMPT_CACHE=0` opts out.
fn enabled() -> bool {
    match std::env::var("CODETETHER_BEDROCK_PROMPT_CACHE") {
        Ok(value) => !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "0" | "false" | "no" | "off"
        ),
        Err(_) => true,
    }
}

/// Anthropic Claude models on Bedrock support `cachePoint` blocks.
fn supported(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains("anthropic") || id.contains("claude")
}
