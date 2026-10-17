//! Parse Converse JSON and construct the provider-neutral response.
use super::*;

/// Parse a Bedrock Converse API response JSON string into a
/// [`CompletionResponse`].
///
/// # Errors
///
/// Returns [`anyhow::Error`] if the string is not valid JSON in the expected
/// Converse response shape.
///
/// # Examples
///
/// ```rust
/// use codetether_bedrock_protocol::parse_converse_response;
/// use codetether_provider_core::{ContentPart, Role};
///
/// let body = r#"{"output":{"message":{"role":"assistant","content":[{"text":"hi"}]}},"stopReason":"end_turn"}"#;
/// let resp = parse_converse_response(body).unwrap();
/// assert!(matches!(resp.message.role, Role::Assistant));
/// assert!(matches!(&resp.message.content[0], ContentPart::Text { text } if text == "hi"));
/// ```
///
/// # Arguments
/// * `text` - JSON body from a Converse response.
///
/// # Returns
/// Content, token usage, and finish reason in provider-neutral form.
pub fn parse_converse_response(text: &str) -> Result<CompletionResponse> {
    let response: ConverseResponse = serde_json::from_str(text).context(format!(
        "Failed to parse Bedrock response: {}",
        codetether_util::truncate_bytes_safe(text, 300)
    ))?;

    let (content, has_tool_calls) = content::content(&response);

    let finish_reason = finish::finish(&response, &content, has_tool_calls)?;

    let usage = response.usage.as_ref();

    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Usage {
            prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
            completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
            total_tokens: usage.map(|u| u.total_tokens).unwrap_or(0),
            cache_read_tokens: None,
            cache_write_tokens: None,
        },
        finish_reason,
    })
}
