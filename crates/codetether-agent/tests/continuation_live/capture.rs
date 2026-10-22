//! Capture public text and proposed tool calls, excluding hidden reasoning.
use codetether_agent::provider::{CompletionResponse, ContentPart};
pub(super) fn has_calls(capture: &serde_json::Value) -> bool {
    capture["calls"]
        .as_array()
        .is_some_and(|calls| !calls.is_empty())
}
pub(super) fn finish(captures: &[serde_json::Value], path: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        captures.iter().all(has_calls),
        "model did not propose an action; inspect capture"
    );
    eprintln!(
        "Captured {} live continuations; no tool calls executed. Semantic assessment required: {path}",
        captures.len()
    );
    Ok(())
}
pub(super) fn record(end: usize, model: &str, response: CompletionResponse) -> serde_json::Value {
    let mut text = Vec::new();
    let mut calls = Vec::new();
    for part in response.message.content {
        match part {
            ContentPart::Text { text: value } => text.push(value),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => calls.push(serde_json::json!({"id":id,"name":name,"arguments":arguments})),
            _ => {}
        }
    }
    serde_json::json!({"end":end,"model":model,"tools_executed":0,"text":text,"calls":calls,"finish_reason":format!("{:?}",response.finish_reason)})
}
