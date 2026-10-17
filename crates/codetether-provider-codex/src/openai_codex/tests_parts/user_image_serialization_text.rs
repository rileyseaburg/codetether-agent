#[test]
fn user_image_serialization_keeps_text_only_wire_shape() {
    for responses in [true, false] {
        let message = Message {
            role: Role::User,
            content: vec![
                ContentPart::Text { text: "one".into() },
                ContentPart::Text { text: "two".into() },
            ],
        };
        let wire = serialized_user_image_message(message, responses);
        let expected = if responses {
            json!({ "type": "message", "role": "user", "content": [
                { "type": "input_text", "text": "one\ntwo" }
            ] })
        } else {
            json!({ "role": "user", "content": "one\ntwo" })
        };
        assert_eq!(wire, expected);
    }
}
