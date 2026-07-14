#[test]
fn extracts_responses_instructions_from_system_messages() {
    let messages = vec![
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "first system block".to_string(),
            }],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "user message".to_string(),
            }],
        },
        Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "second system block".to_string(),
            }],
        },
    ];

    let instructions = OpenAiCodexProvider::extract_responses_instructions(&messages);
    assert_eq!(instructions, "first system block\n\nsecond system block");
}
