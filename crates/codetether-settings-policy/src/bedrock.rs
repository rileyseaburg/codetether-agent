//! Bedrock service-tier and thinking-effort labels and cycles.
/// Normalize a Bedrock service-tier label.
/// # Arguments
/// * `current` - Current provider value.
/// # Returns
/// standard, priority or default.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::bedrock_service_tier_label(""), "default");
/// ```
pub fn bedrock_service_tier_label(current: &str) -> &'static str {
    match current {
        "standard" => "standard",
        "priority" => "priority",
        _ => "default",
    }
}
/// Advance the service-tier setting.
/// # Arguments
/// * `current` - Normalized service-tier label.
/// # Returns
/// The next override or None for the default.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::next_bedrock_service_tier("priority"), None);
/// ```
pub fn next_bedrock_service_tier(current: &str) -> Option<String> {
    match current {
        "default" => Some("standard".to_string()),
        "standard" => Some("priority".to_string()),
        _ => None,
    }
}
/// Normalize a Bedrock thinking-effort label.
/// # Arguments
/// * `current` - Current provider value.
/// # Returns
/// low, high or medium.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::bedrock_thinking_effort_label(""), "medium");
/// ```
pub fn bedrock_thinking_effort_label(current: &str) -> &'static str {
    match current {
        "low" => "low",
        "high" => "high",
        _ => "medium",
    }
}
/// Advance the Bedrock effort setting.
/// # Arguments
/// * `current` - Normalized effort label.
/// # Returns
/// The next effort in the medium, low, high cycle.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::next_bedrock_thinking_effort("low"), "high");
/// ```
pub fn next_bedrock_thinking_effort(current: &str) -> &'static str {
    match current {
        "medium" => "low",
        "low" => "high",
        _ => "medium",
    }
}
