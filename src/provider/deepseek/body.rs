//! Build the HTTP request body for a DeepSeek completion.

use crate::provider::CompletionRequest;
use serde_json::{Value, json};

use super::convert;
use super::convert_tools;

pub(super) fn build(req: &CompletionRequest) -> Value {
    let messages = convert::messages(&req.messages);
    let tools = convert_tools::tools(&req.tools);
    let mut body = json!({"model": req.model, "messages": messages});
    if !tools.is_empty() {
        body["tools"] = json!(tools);
    }
    if let Some(max) = req.max_tokens {
        body["max_tokens"] = json!(max);
    }
    body["thinking"] = json!({"type": "enabled"});
    body["reasoning_effort"] = json!("high");
    body
}
