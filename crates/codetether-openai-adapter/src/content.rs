//! OpenAI-compatible content conversion.
use crate::parse_openai_content_part;

/// Parse optional string, object or array message content.
/// # Arguments
/// content is the optional incoming content value.
/// # Returns
/// Recognized content parts in their original order.
/// # Examples
/// ```
/// assert!(codetether_openai_adapter::parse_openai_content_parts(&None).is_empty());
/// ```
pub fn parse_openai_content_parts(
    content: &Option<serde_json::Value>,
) -> Vec<codetether_provider_core::ContentPart> {
    let Some(value) = content else {
        return Vec::new();
    };

    match value {
        serde_json::Value::String(text) => {
            vec![codetether_provider_core::ContentPart::Text { text: text.clone() }]
        }
        serde_json::Value::Array(parts) => {
            parts.iter().filter_map(parse_openai_content_part).collect()
        }
        serde_json::Value::Object(_) => parse_openai_content_part(value).into_iter().collect(),
        _ => Vec::new(),
    }
}

/// Join text content for a tool-result message.
/// # Arguments
/// content is the incoming tool message content.
/// # Returns
/// Newline-joined text parts.
/// # Examples
/// ```
/// assert_eq!(codetether_openai_adapter::parse_openai_tool_content(&None), "");
/// ```
pub fn parse_openai_tool_content(content: &Option<serde_json::Value>) -> String {
    parse_openai_content_parts(content)
        .into_iter()
        .filter_map(|part| match part {
            codetether_provider_core::ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
