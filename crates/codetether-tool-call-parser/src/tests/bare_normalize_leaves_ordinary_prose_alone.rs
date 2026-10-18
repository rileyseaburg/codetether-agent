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
fn normalize_leaves_ordinary_prose_alone() {
    let r = normalize_text("I considered running {something} but decided not to.");
    assert_eq!(r.finish_reason, FinishReason::Stop);
    assert!(
        r.message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::Text { .. }))
    );
}
