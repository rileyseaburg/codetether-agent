impl OpenAiCodexProvider {
    fn convert_message(message: &Message) -> Value {
        match message.role {
            Role::Tool => Self::convert_tool_message(message),
            Role::Assistant => Self::convert_assistant_message(message),
            Role::System => json!({
                "role": "system",
                "content": Self::message_text(message, "\n"),
            }),
            Role::User => json!({
                "role": "user",
                "content": Self::message_text(message, "\n"),
            }),
        }
    }
}
