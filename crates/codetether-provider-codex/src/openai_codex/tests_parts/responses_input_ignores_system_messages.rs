#[test]
fn responses_input_ignores_system_messages() {
    let messages = vec![
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "system".to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "user".to_string(),
            }],
        },
    ];

    let input = OpenAiCodexProvider::convert_messages_to_responses_input(&messages);
    assert_eq!(input.len(), 1);
    assert_eq!(input[0].get("role").and_then(Value::as_str), Some("user"));
}
