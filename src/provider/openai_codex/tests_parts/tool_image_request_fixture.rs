fn tool_image_wire_request(success: bool) -> CompletionRequest {
    let mut request = ws_tool_request();
    for (call_id, url) in [
        ("pixel-call", USER_IMAGE_DATA_URL),
        ("remote-call", USER_IMAGE_REMOTE_URL),
    ] {
        request.messages.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: call_id.into(),
                name: "image".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        });
        let metadata = std::collections::HashMap::from([(
            "image_data_url".to_string(),
            json!({"data_url": url}),
        )]);
        request
            .messages
            .push(crate::session::helper::image_tool_message_for_test(
                call_id.into(),
                "image",
                success,
                "inspected image".into(),
                Some(&metadata),
            ));
    }
    request
}
