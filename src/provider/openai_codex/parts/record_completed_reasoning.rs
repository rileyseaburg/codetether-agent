impl OpenAiCodexProvider {
    fn record_completed_reasoning(response: Option<&Value>, chunks: &mut Vec<StreamChunk>) {
        let Some(output) = response
            .and_then(|value| value.get("output"))
            .and_then(Value::as_array)
        else {
            return;
        };
        for item in output {
            super::codex_reasoning::push_item_if_reasoning(item, chunks);
        }
    }
}
