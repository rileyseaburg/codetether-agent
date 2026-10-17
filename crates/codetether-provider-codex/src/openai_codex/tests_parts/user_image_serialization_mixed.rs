#[test]
fn user_image_serialization_preserves_mixed_order() {
    for responses in [true, false] {
        let message = Message {
            role: Role::User,
            content: vec![
                ContentPart::Text {
                    text: "before".into(),
                },
                user_image_part(USER_IMAGE_DATA_URL),
                ContentPart::Text {
                    text: "between".into(),
                },
                user_image_part(USER_IMAGE_REMOTE_URL),
                ContentPart::Text {
                    text: "after".into(),
                },
            ],
        };
        let wire = serialized_user_image_message(message, responses);
        let expected = if responses {
            json!([
                { "type": "input_text", "text": "before" },
                { "type": "input_image", "image_url": USER_IMAGE_DATA_URL },
                { "type": "input_text", "text": "between" },
                { "type": "input_image", "image_url": USER_IMAGE_REMOTE_URL },
                { "type": "input_text", "text": "after" },
            ])
        } else {
            json!([
                { "type": "text", "text": "before" },
                { "type": "image_url", "image_url": { "url": USER_IMAGE_DATA_URL } },
                { "type": "text", "text": "between" },
                { "type": "image_url", "image_url": { "url": USER_IMAGE_REMOTE_URL } },
                { "type": "text", "text": "after" },
            ])
        };
        assert_eq!(wire["role"], "user");
        assert_eq!(wire["content"], expected);
        for node in wire["content"].as_array().unwrap() {
            if let Some(text) = node["text"].as_str() {
                assert!(!text.contains("data:image/"));
            }
        }
    }
}
