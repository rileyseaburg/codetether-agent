use serde_json::Value;

pub(crate) fn first_user_text(payload: &Value) -> Option<String> {
    if payload.get("type")?.as_str()? != "message" {
        return None;
    }
    if payload.get("role")?.as_str()? != "user" {
        return None;
    }
    payload
        .get("content")?
        .as_array()?
        .iter()
        .find_map(content_text)
}

fn content_text(item: &Value) -> Option<String> {
    match item.get("type").and_then(Value::as_str)? {
        "input_text" | "output_text" => clean_text(item.get("text")?.as_str()?),
        _ => None,
    }
}

fn clean_text(text: &str) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}
