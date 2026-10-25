//! Validate that the second tool result continued the original conversation.
use codetether_tool_core::ToolResult;

fn load(result: &ToolResult) -> serde_json::Value {
    let path = result.metadata["refinement_path"].as_str().unwrap();
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

pub(super) fn verify(first: &ToolResult, second: &ToolResult) {
    let first_state = load(first);
    let second_state = load(second);
    assert_eq!(
        first_state["conversation"]["conversationId"],
        second_state["conversation"]["conversationId"]
    );
    assert_ne!(
        first_state["conversation"]["responseId"],
        second_state["conversation"]["responseId"]
    );
}
