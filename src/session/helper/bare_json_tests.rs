//! Tests for [`super::super::bare_json`] and the tool-call normalizer salvage
//! path that fixes the swarm "0 usable changes" failure.

use super::super::bare_json::extract_bare_json_tool_call;
use super::super::markup::normalize_textual_tool_calls;
use crate::provider::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, ToolDefinition, Usage,
};
use serde_json::json;
use std::collections::HashSet;

fn bash_tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition { name: "bash".into(), description: "shell".into(), parameters: json!({}) }]
}

fn normalize_text(text: &str) -> CompletionResponse {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: text.to_string() }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    normalize_textual_tool_calls(response, &bash_tools())
}

#[test]
fn extract_bare_json_handles_keys_fences_and_rejections() {
    let allowed: HashSet<&str> = ["bash"].into_iter().collect();
    assert!(extract_bare_json_tool_call(r#"{"name":"bash","input":{"command":"ls"}}"#, &allowed).is_some());
    assert!(extract_bare_json_tool_call("```json\n{\"name\":\"bash\",\"args\":{}}\n```", &allowed).is_some());
    assert!(extract_bare_json_tool_call(r#"{"name":"nope"}"#, &allowed).is_none());
    assert!(extract_bare_json_tool_call("prose {x} more", &allowed).is_none());
}

#[test]
fn normalize_salvages_bare_json_without_markup() {
    let r = normalize_text(r#"{"name":"bash","input":{"command":"ls -la"}}"#);
    assert_eq!(r.finish_reason, FinishReason::ToolCalls);
    assert!(r.message.content.iter().any(|p| matches!(p, ContentPart::ToolCall { name, .. } if name == "bash")));
    assert!(r.message.content.iter().all(|p| !matches!(p, ContentPart::Text { .. })));
}

#[test]
fn normalize_leaves_ordinary_prose_alone() {
    let r = normalize_text("I considered running {something} but decided not to.");
    assert_eq!(r.finish_reason, FinishReason::Stop);
    assert!(r.message.content.iter().any(|p| matches!(p, ContentPart::Text { .. })));
}
