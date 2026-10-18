//! Original parsing regression case.
//! Tests for [`super::super::bare_json`] and the tool-call normalizer salvage
//! path that fixes the swarm "0 usable changes" failure.

use crate::extract_bare_json_tool_call;

use crate::normalize_textual_tool_calls;

use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, ToolDefinition, Usage,
};

use serde_json::json;

use super::bare_support::*;
use std::collections::HashSet;

#[test]
fn normalize_salvages_bare_json_without_markup() {
    let r = normalize_text(r#"{"name":"bash","input":{"command":"ls -la"}}"#);
    assert_eq!(r.finish_reason, FinishReason::ToolCalls);
    assert!(
        r.message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { name, .. } if name == "bash"))
    );
    assert!(
        r.message
            .content
            .iter()
            .all(|p| !matches!(p, ContentPart::Text { .. }))
    );
}
