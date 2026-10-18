//! Extract JSON model responses, including surrounding prose.

pub(crate) fn extract_json_payload<T: serde::de::DeserializeOwned>(text: &str) -> Option<T> {
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
