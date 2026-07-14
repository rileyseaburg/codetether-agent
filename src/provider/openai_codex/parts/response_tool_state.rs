impl OpenAiCodexProvider {
    fn response_tool_state<'a>(
        parser: &'a mut ResponsesSseParser,
        key: &str,
        call_id: String,
    ) -> &'a mut ResponsesToolState {
        let state = parser
            .tools
            .entry(key.to_string())
            .or_insert_with(|| ResponsesToolState {
                call_id: call_id.clone(),
                ..ResponsesToolState::default()
            });
        if state.call_id.is_empty() {
            state.call_id = call_id;
        }
        state
    }
}
