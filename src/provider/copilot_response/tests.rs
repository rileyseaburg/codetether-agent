//! Tests for Copilot response conversion edge cases.

use super::{CopilotResponse, to_completion_response};
use crate::provider::{ContentPart, FinishReason};

#[test]
fn collects_tool_calls_from_later_choices() {
    let wire = r#"{
      "choices": [
        {"message": {"content": "I'll inspect."}, "finish_reason": "stop"},
        {"message": {"tool_calls": [{"id": "call_1", "type": "function",
          "function": {"name": "grep", "arguments": ""}}]},
         "finish_reason": "tool_calls"}
      ],
      "usage": {"prompt_tokens": 1, "completion_tokens": 2, "total_tokens": 3}
    }"#;
    let response: CopilotResponse = serde_json::from_str(wire).unwrap();
    let complete = to_completion_response(response).unwrap();
    assert_eq!(complete.finish_reason, FinishReason::ToolCalls);
    assert_eq!(complete.usage.total_tokens, 3);
    assert!(matches!(
        complete.message.content[0],
        ContentPart::Text { .. }
    ));
    match &complete.message.content[1] {
        ContentPart::ToolCall {
            name, arguments, ..
        } => {
            assert_eq!(name, "grep");
            assert_eq!(arguments, "{}");
        }
        other => panic!("expected tool call, got {other:?}"),
    }
}

#[test]
fn missing_arguments_deserialize_as_empty_object() {
    let wire = r#"{"choices":[{"message":{"tool_calls":[{"id":"c",
      "function":{"name":"list"}}]}}]}"#;
    let response: CopilotResponse = serde_json::from_str(wire).unwrap();
    let complete = to_completion_response(response).unwrap();
    assert!(
        matches!(&complete.message.content[0], ContentPart::ToolCall { arguments, .. } if arguments == "{}")
    );
}
