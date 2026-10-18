//! Shared original parsing-test fixtures.
//! Tests for [`super::super::bare_json`] and the tool-call normalizer salvage
//! path that fixes the swarm "0 usable changes" failure.

use crate::extract_bare_json_tool_call;

use crate::normalize_textual_tool_calls;

use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, ToolDefinition, Usage,
};

use serde_json::json;

use std::collections::HashSet;

pub(super) fn bash_tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "bash".into(),
        description: "shell".into(),
        parameters: json!({}),
    }]
}

pub(super) fn normalize_text(text: &str) -> CompletionResponse {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: text.to_string(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    normalize_textual_tool_calls(response, &bash_tools())
}
