//! Build the HTTP request body for a DeepSeek completion.
//!
//! V4 models support `thinking` and `reasoning_effort`.
//! Legacy models do not and must omit them.

use crate::provider::CompletionRequest;
use serde_json::{Value, json};

use super::convert;
use super::convert_tools;

/// Whether the given model ID is a DeepSeek V4 model.
fn is_v4(model: &str) -> bool {
    model.contains("v4")
}

pub(super) fn build(req: &CompletionRequest) -> Value {
    let messages = convert::messages(&req.messages);
    let tools = convert_tools::tools(&req.tools);
    let mut body = json!({"model": req.model, "messages": messages});

    if !tools.is_empty() {
        body["tools"] = json!(tools);
        body["tool_choice"] = json!("auto");
    }

    if let Some(max) = req.max_tokens {
        body["max_tokens"] = json!(max);
    }

    let temperature = req.temperature.unwrap_or(1.0);
    body["temperature"] = json!(temperature);

    if is_v4(&req.model) {
        body["thinking"] = json!({"type": "enabled"});
        body["reasoning_effort"] = json!("high");
    }

    body
}
