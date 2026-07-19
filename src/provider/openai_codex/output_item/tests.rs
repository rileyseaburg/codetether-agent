use super::checkpoint;
use crate::provider::ContentPart;
use serde_json::json;

#[test]
fn converts_completed_message() {
    let part = checkpoint(&json!({
        "type": "message",
        "content": [{"type": "output_text", "text": "done"}]
    }));
    assert!(matches!(part, Some(ContentPart::Text { text }) if text == "done"));
}

#[test]
fn converts_completed_tool_call() {
    let part = checkpoint(&json!({
        "type": "function_call", "call_id": "call_1",
        "name": "read", "arguments": "{\"path\":\"a\"}"
    }));
    assert!(
        matches!(part, Some(ContentPart::ToolCall { id, name, .. }) if id == "call_1" && name == "read")
    );
}

#[test]
fn ignores_incomplete_reasoning_checkpoint() {
    assert!(checkpoint(&json!({"type": "reasoning"})).is_none());
}
