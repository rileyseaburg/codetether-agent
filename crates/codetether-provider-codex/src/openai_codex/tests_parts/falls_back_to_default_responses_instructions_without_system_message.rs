#[test]
fn falls_back_to_default_responses_instructions_without_system_message() {
    let messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "only user".to_string(),
        }],
    }];

    let instructions = OpenAiCodexProvider::extract_responses_instructions(&messages);
    assert_eq!(instructions, DEFAULT_RESPONSES_INSTRUCTIONS);
}
