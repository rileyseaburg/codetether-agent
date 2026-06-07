use serde_json::Value;

pub(crate) fn count_response_item(payload: &Value) -> bool {
    match payload.get("type").and_then(Value::as_str) {
        Some("message") => count_message(payload),
        Some("reasoning") => count_reasoning(payload),
        Some("function_call") => payload.get("name").and_then(Value::as_str).is_some(),
        Some("function_call_output") => payload.get("call_id").and_then(Value::as_str).is_some(),
        Some(_) | None => false,
    }
}

fn count_message(payload: &Value) -> bool {
    if !matches!(
        payload.get("role").and_then(Value::as_str),
        Some("user" | "assistant")
    ) {
        return false;
    }
    payload
        .get("content")
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(count_message_content))
}

fn count_message_content(item: &Value) -> bool {
    match item.get("type").and_then(Value::as_str) {
        Some("input_text" | "output_text") => item
            .get("text")
            .and_then(Value::as_str)
            .is_some_and(|text| !text.trim().is_empty()),
        Some("input_image") => item
            .get("image_url")
            .and_then(Value::as_str)
            .is_some_and(|url| !url.is_empty()),
        Some(_) | None => false,
    }
}

fn count_reasoning(payload: &Value) -> bool {
    payload
        .get("content")
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
        || payload
            .get("summary")
            .and_then(Value::as_array)
            .is_some_and(|items| !items.is_empty())
}
