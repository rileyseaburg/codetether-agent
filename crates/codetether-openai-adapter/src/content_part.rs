//! OpenAI-compatible content part conversion.

/// Parse one supported JSON content part.
/// # Arguments
/// value is an incoming content part.
/// # Returns
/// Text/image content, or None for an unrecognized part.
/// # Examples
/// ```
/// assert!(codetether_openai_adapter::parse_openai_content_part(&serde_json::json!(null)).is_none());
/// ```
pub fn parse_openai_content_part(
    value: &serde_json::Value,
) -> Option<codetether_provider_core::ContentPart> {
    let obj = value.as_object()?;
    let part_type = obj
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("text");

    match part_type {
        "text" | "input_text" => obj
            .get("text")
            .and_then(serde_json::Value::as_str)
            .map(|text| codetether_provider_core::ContentPart::Text {
                text: text.to_string(),
            }),
        "image_url" => obj
            .get("image_url")
            .and_then(serde_json::Value::as_object)
            .and_then(|image| image.get("url"))
            .and_then(serde_json::Value::as_str)
            .map(|url| codetether_provider_core::ContentPart::Image {
                url: url.to_string(),
                mime_type: None,
            }),
        _ => obj
            .get("text")
            .and_then(serde_json::Value::as_str)
            .map(|text| codetether_provider_core::ContentPart::Text {
                text: text.to_string(),
            }),
    }
}
