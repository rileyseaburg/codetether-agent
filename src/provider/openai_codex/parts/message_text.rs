impl OpenAiCodexProvider {
    fn message_text(message: &Message, separator: &str) -> String {
        message
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(separator)
    }
}
