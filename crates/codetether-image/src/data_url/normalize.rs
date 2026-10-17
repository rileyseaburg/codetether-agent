//! Normalize wrapped pasted base64 without copying unwrapped input.
use std::borrow::Cow;
pub(super) fn normalized_payload(payload: &str) -> Option<Cow<'_, str>> {
    let payload = payload.trim();
    if payload.is_empty() {
        return None;
    }
    if !payload.as_bytes().iter().any(u8::is_ascii_whitespace) {
        return Some(Cow::Borrowed(payload));
    }
    let payload: String = payload.split_ascii_whitespace().collect();
    (!payload.is_empty()).then_some(Cow::Owned(payload))
}
