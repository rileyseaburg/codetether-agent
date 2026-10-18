//! Extract planner JSON from plain or wrapped responses.
use serde::de::DeserializeOwned;

/// Parse planner JSON, including object or array text wrapped in prose.
/// # Arguments
/// * `text` - Entire planner response.
/// # Returns
/// The first successfully parsed full response, object span or array span.
/// # Examples
/// ```
/// let value: Option<serde_json::Value> = codetether_relay_plan::extract_json_payload("prefix {\"ok\":true} suffix");
/// assert_eq!(value.unwrap()["ok"], true);
/// ```
pub fn extract_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
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

    if let (Some(start), Some(end)) = (trimmed.find('['), trimmed.rfind(']'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}
