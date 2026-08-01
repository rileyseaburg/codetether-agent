//! Extraction of the answer text from one Gemini wire event.

use serde_json::Value;

/// Reads the response text slot from a single wire event.
///
/// Wire format: each line is a JSON array of events shaped
/// `["wrb.fr", key_or_null, inner_json_str, ...]`, and the inner JSON carries
/// the response text at `[4][0][1][0]`.
///
/// Returns `None` when the event is not a text-bearing frame.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::frame::event_text;
/// use serde_json::json;
///
/// let inner = json!([null, null, null, null, [[null, ["hello"]]]]).to_string();
/// let event = json!(["wrb.fr", null, inner]);
/// assert_eq!(event_text(&event).as_deref(), Some("hello"));
///
/// // Non-text frames are ignored.
/// assert!(event_text(&json!(["wrb.fr", null, "not-json"])).is_none());
/// ```
pub fn event_text(event: &Value) -> Option<String> {
    let inner = event.get(2)?.as_str()?;
    if !inner.starts_with('[') {
        return None;
    }
    let value = serde_json::from_str::<Value>(inner).ok()?;
    value
        .get(4)?
        .get(0)?
        .get(1)?
        .get(0)?
        .as_str()
        .map(str::to_string)
}
