impl OpenAiCodexProvider {
    fn extract_responses_instructions(messages: &[Message]) -> String {
        let instructions = messages
            .iter()
            .filter(|msg| matches!(msg.role, Role::System))
            .filter_map(|msg| {
                let text = msg
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                (!text.is_empty()).then_some(text)
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        if instructions.is_empty() {
            DEFAULT_RESPONSES_INSTRUCTIONS.to_string()
        } else {
            instructions
        }
    }
}
