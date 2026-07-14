impl OpenAiCodexProvider {
    fn extract_json_string(value: Option<&Value>) -> Option<String> {
        match value {
            Some(Value::String(text)) => Some(text.clone()),
            Some(other) => serde_json::to_string(other).ok(),
            None => None,
        }
    }
}
