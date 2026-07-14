impl OpenAiCodexProvider {
    fn convert_messages_to_responses_input(messages: &[Message]) -> Vec<Value> {
        let mut input = Vec::new();
        let mut known_calls = std::collections::HashSet::new();
        for message in messages {
            Self::append_responses_message(message, &mut input, &mut known_calls);
        }
        stream_recovery::request_anchor::ensure(input)
    }
}
