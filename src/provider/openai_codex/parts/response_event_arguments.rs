impl OpenAiCodexProvider {
    fn response_event_arguments(event: &Value, final_arguments: bool) -> Option<String> {
        if final_arguments
            && let Some(arguments) = Self::extract_json_string(event.get("arguments"))
        {
            return Some(arguments);
        }
        event
            .get("delta")
            .and_then(Value::as_str)
            .map(str::to_string)
    }
}
