//! Existing plain export regression coverage.
use crate::extract_message_text;
use codetether_provider_core::ContentPart;

#[test]
fn extract_message_text_keeps_tool_markers_for_plain_export() {
    let content = vec![
        ContentPart::Text {
            text: "hello".to_string(),
        },
        ContentPart::ToolCall {
            id: "call_1".to_string(),
            name: "read".to_string(),
            arguments: "{\"path\":\"src/main.rs\"}".to_string(),
            thought_signature: None,
        },
    ];

    assert!(extract_message_text(&content).contains("Tool call: read"));
}
