use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::json;

#[test]
fn tagged_content_variants_roundtrip_without_wire_changes() {
    for wire in [
        json!({"type": "text", "text": "hello"}),
        json!({"type": "image", "url": "image.png", "mime_type": null}),
        json!({"type": "file", "path": "file.txt", "mime_type": "text/plain"}),
        json!({"type": "tool_call", "id": "c1", "name": "lookup", "arguments": "{}"}),
        json!({"type": "tool_call", "id": "c2", "name": "lookup", "arguments": "{}",
            "thought_signature": "signed"}),
        json!({"type": "tool_result", "tool_call_id": "c1", "content": "ok"}),
        json!({"type": "thinking", "text": "reasoning", "signature": null}),
    ] {
        let part: ContentPart = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(part).unwrap(), wire);
    }
}

#[test]
fn message_keeps_content_order_and_tool_role() {
    let wire = json!({"role": "tool", "content": [
        {"type": "tool_result", "tool_call_id": "c1", "content": "ok"},
        {"type": "text", "text": "after tool"},
    ]});
    let message: Message = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(message.role, Role::Tool);
    assert!(matches!(message.content[0], ContentPart::ToolResult { .. }));
    assert!(matches!(message.content[1], ContentPart::Text { .. }));
    assert_eq!(serde_json::to_value(message).unwrap(), wire);
}
