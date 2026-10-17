//! Relay context parse json payload.
use super::*;

/// Parse JSON directly or from the outermost object in surrounding text.
///
/// # Arguments
/// * `text` - Candidate JSON payload.
///
/// # Returns
/// The requested deserialized type when parsing succeeds.
///
/// # Examples
/// ```
/// use codetether_autochat_core::shared_context::{ContextDelta, parse_json_payload};
/// assert!(parse_json_payload::<ContextDelta>("{}").is_some());
/// ```
pub fn parse_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
    let trimmed = text.trim();
    if let Ok(value) = serde_json::from_str::<T>(trimmed) {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}
