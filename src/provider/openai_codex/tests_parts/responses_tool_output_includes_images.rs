#[test]
fn responses_tool_output_includes_images() {
    let message = Message {
        role: Role::Tool,
        content: vec![
            ContentPart::ToolResult {
                tool_call_id: "call-1".into(),
                content: "generated".into(),
            },
            ContentPart::Image {
                url: "data:image/png;base64,eA==".into(),
                mime_type: Some("image/png".into()),
            },
        ],
    };
    let mut input = Vec::new();
    let known = std::collections::HashSet::from(["call-1".to_string()]);
    OpenAiCodexProvider::append_responses_tool(&message, &mut input, &known);
    assert_eq!(
        input,
        vec![json!({
            "type": "function_call_output",
            "call_id": "call-1",
            "output": [
                {
                    "type": "input_image",
                    "image_url": "data:image/png;base64,eA==",
                    "detail": "auto"
                },
                {"type": "input_text", "text": "generated"}
            ]
        })]
    );
}