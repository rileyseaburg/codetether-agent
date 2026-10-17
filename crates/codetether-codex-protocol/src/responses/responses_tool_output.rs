//! responses tool output.
use crate::*;
impl CodexProtocol {
    /// Encode tool text and any image results.
    ///
    /// # Arguments
    /// Accepts provider-neutral message/tool content or the mutable request
    /// projection and known call identifiers used during conversion.
    ///
    /// # Returns
    /// The encoded value or, for append methods, updates the supplied projection.
    ///
    /// # Examples
    /// See [CodexProtocol::convert_messages_to_responses_input] for the request encoding entry point.
    pub fn responses_tool_output(parts: &[ContentPart], text: &str) -> Value {
        let mut content = parts
            .iter()
            .filter_map(|part| match part {
                ContentPart::Image { url, .. } => Some(json!({
                    "type": "input_image",
                    "image_url": url,
                    "detail": "auto",
                })),
                _ => None,
            })
            .collect::<Vec<_>>();
        if content.is_empty() {
            return Value::String(text.to_string());
        }
        content.push(json!({"type": "input_text", "text": text}));
        Value::Array(content)
    }
}
