use super::Continuation;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
mod layout;

/// Build the JSON value of Gemini's image-generation `f.req` field.
///
/// # Arguments
/// * `prompt` - Nonblank generation or refinement instructions.
/// * `previous` - Validated server state to continue, or `None` for a fresh chat.
///
/// # Returns
/// An outer `[null, encoded_inner]` JSON array. The inner array has 99 slots.
/// This is not a URL-encoded form; transport must add its own session token.
///
/// # Errors
/// Returns fixed safe errors for blank prompts or invalid continuation IDs.
///
/// # Examples
/// ```
/// use codetether_gemini_web_protocol::images::build_payload;
/// let outer: serde_json::Value = serde_json::from_str(
///     &build_payload("Draw a lake", None).unwrap()).unwrap();
/// let inner: Vec<serde_json::Value> = serde_json::from_str(outer[1].as_str().unwrap()).unwrap();
/// assert_eq!(inner.len(), 99);
/// assert_eq!(inner[0][0], "Draw a lake");
/// ```
pub fn build_payload(prompt: &str, previous: Option<&Continuation>) -> Result<String> {
    ensure!(!prompt.trim().is_empty(), "image_prompt_empty");
    if let Some(state) = previous {
        state.validate()?;
    }
    let mut inner = layout::new(prompt);
    if let Some(state) = previous {
        let mut metadata = vec![Value::Null; 10];
        metadata[0] = json!(state.conversation_id);
        metadata[1] = json!(state.response_id);
        metadata[2] = json!(state.choice_id);
        metadata[9] = json!("");
        inner[2] = json!(metadata);
        let mut context = vec![Value::Null; 8];
        context[0] = json!(state.conversation_id);
        context[7] = json!("");
        inner[71] = json!(context);
    }
    Ok(json!([null, Value::Array(inner).to_string()]).to_string())
}
