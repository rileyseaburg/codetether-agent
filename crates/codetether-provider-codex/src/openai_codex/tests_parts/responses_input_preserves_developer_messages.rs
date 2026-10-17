#[test]
fn responses_input_preserves_developer_messages() {
    let messages = vec![Message {
        role: Role::Developer,
        content: vec![ContentPart::Text {
            text: "interrupted".into(),
        }],
    }];
    let input = OpenAiCodexProvider::convert_messages_to_responses_input(&messages);
    assert_eq!(input.len(), 1);
    assert_eq!(
        input[0].get("role").and_then(Value::as_str),
        Some("developer")
    );
}
