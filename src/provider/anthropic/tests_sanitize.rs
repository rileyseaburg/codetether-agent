//! Tests for orphan tool-result filtering (Anthropic error 2013 guard).

use crate::provider::{ContentPart, Message, Role};

fn has_tool_result(c: &[serde_json::Value]) -> bool {
    c.iter().any(|m| {
        m["content"]
            .as_array()
            .is_some_and(|c| c.iter().any(|b| b["type"] == "tool_result"))
    })
}

fn tool_result_msg(id: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.to_string(),
            content: "out".to_string(),
        }],
    }
}

#[test]
fn orphan_tool_result_without_matching_call_is_dropped() {
    // tool_use was dropped (e.g. by compression/undo); result is orphaned.
    let messages = vec![
        super::test_support::user_message("hi"),
        tool_result_msg("orphan-1"),
    ];
    let (_s, converted) = super::convert::messages(&messages, false);
    assert!(!has_tool_result(&converted), "orphan should be dropped");
}

#[test]
fn paired_tool_result_is_preserved() {
    let messages = vec![
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call-1".to_string(),
                name: "get_weather".to_string(),
                arguments: "{}".to_string(),
                thought_signature: None,
            }],
        },
        tool_result_msg("call-1"),
    ];
    let (_s, converted) = super::convert::messages(&messages, false);
    assert!(has_tool_result(&converted), "match must be preserved");
}
