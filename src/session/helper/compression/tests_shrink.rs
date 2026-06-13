//! Tests for payload shrinking inside terminal truncation.

use crate::provider::{ContentPart, Message, Role};
use crate::session::helper::token::estimate_request_tokens;

use super::terminal::terminal_truncate_messages;

fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

fn tool_result(content: String) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: "call_1".to_string(),
            content,
        }],
    }
}

#[test]
fn terminal_truncate_shrinks_oversized_tail_when_message_count_is_short() {
    let huge_output = format!("head\n{}\ntail", "x".repeat(200_000));
    let mut messages = vec![user("inspect the logs"), tool_result(huge_output)];
    let before = estimate_request_tokens("", &messages, &[]);

    let dropped = terminal_truncate_messages(&mut messages, "", &[], 4, 6_000);
    let after = estimate_request_tokens("", &messages, &[]);

    assert!(dropped > 0);
    assert!(after < before);
    assert!(after <= 6_000, "after={after}");
    assert_eq!(messages.len(), 3);
    assert!(matches!(messages[0].role, Role::Assistant));

    let ContentPart::ToolResult { content, .. } = &messages[2].content[0] else {
        panic!("expected tool result");
    };
    assert!(content.contains("terminal context fallback truncated tool_result"));
    assert!(content.contains("head"));
    assert!(content.contains("tail"));
}
