//! Collection of answer candidates from Gemini wire frames.

use super::{frame, placeholder};
use serde_json::Value;

/// Collects every non-placeholder candidate in wire order.
///
/// Order is preserved so callers can break ties toward later frames, which is
/// how genuine cumulative replacements are resolved.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::gemini_web::response_text::collect::candidates;
///
/// let inner = r#"[null,null,null,null,[[null,["hello"]]]]"#;
/// let body = format!("[[\"wrb.fr\",null,{}]]", serde_json::to_string(inner).unwrap());
/// assert_eq!(candidates(&body), vec!["hello".to_string()]);
///
/// // Non-frame lines are ignored.
/// assert!(candidates(")]}'\nnot-a-frame").is_empty());
/// ```
pub fn candidates(raw: &str) -> Vec<String> {
    let mut found = Vec::new();
    let frames = raw
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('['))
        .filter_map(|line| serde_json::from_str::<Value>(line).ok());
    for events in frames {
        for event in events.as_array().into_iter().flatten() {
            if let Some(text) = frame::event_text(event)
                && !text.is_empty()
                && !placeholder::is_placeholder(&text)
            {
                found.push(text);
            }
        }
    }
    found
}
