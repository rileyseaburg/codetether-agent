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
fn extract_bare_json_handles_keys_fences_and_rejections() {
    let allowed: HashSet<&str> = ["bash"].into_iter().collect();
    assert!(
        extract_bare_json_tool_call(r#"{"name":"bash","input":{"command":"ls"}}"#, &allowed)
            .is_some()
    );
    assert!(
        extract_bare_json_tool_call("```json\n{\"name\":\"bash\",\"args\":{}}\n```", &allowed)
            .is_some()
    );
    assert!(extract_bare_json_tool_call(r#"{"name":"nope"}"#, &allowed).is_none());
    assert!(extract_bare_json_tool_call("prose {x} more", &allowed).is_none());
}
