impl OpenAiCodexProvider {
    fn emit_missing_responses_tool_arguments(
        state: &mut ResponsesToolState,
        arguments: String,
        chunks: &mut Vec<StreamChunk>,
    ) {
        let delta = if arguments.starts_with(&state.emitted_arguments) {
            arguments[state.emitted_arguments.len()..].to_string()
        } else if state.emitted_arguments.is_empty() {
            arguments.clone()
        } else if arguments == state.emitted_arguments {
            String::new()
        } else {
            arguments.clone()
        };

        if !delta.is_empty() {
            chunks.push(StreamChunk::ToolCallDelta {
                id: state.call_id.clone(),
                arguments_delta: delta.clone(),
            });
            state.emitted_arguments.push_str(&delta);
        }
    }
}
