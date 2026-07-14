impl OpenAiCodexProvider {
    fn append_responses_message(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &mut std::collections::HashSet<String>,
    ) {
        match message.role {
            Role::System => {}
            Role::User => Self::append_responses_user(message, input),
            Role::Assistant => Self::append_responses_assistant(message, input, known_calls),
            Role::Tool => Self::append_responses_tool(message, input, known_calls),
        }
    }
}
