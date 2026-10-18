//! Provider-specific reasoning-level cycle rules.
/// Display an explicit reasoning override or its default label.
/// # Arguments
/// * `current` - Current override.
/// # Returns
/// The override text or default.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::reasoning_label(None), "default");
/// ```
pub fn reasoning_label(current: Option<String>) -> String {
    current.unwrap_or_else(|| "default".to_string())
}
/// Advance through the active Codex model effort levels.
/// # Arguments
/// * `current` - Current override.
/// * `levels` - Nonempty provider-supported levels.
/// # Returns
/// The next level or None after the last or an unrecognized override.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::next_codex_effort(None, &["low"]), Some("low"));
/// ```
/// # Panics
/// Panics if levels is empty; the caller must reject unsupported models first.
pub fn next_codex_effort<'a>(current: Option<&str>, levels: &[&'a str]) -> Option<&'a str> {
    let next = current
        .and_then(|value| levels.iter().position(|level| *level == value))
        .and_then(|index| levels.get(index + 1))
        .copied();
    current.is_none().then_some(levels[0]).or(next)
}
/// Advance the OpenRouter reasoning override.
/// # Arguments
/// * `current` - Current override.
/// * `levels` - Nonempty ordered provider levels.
/// # Returns
/// The next override; missing/unknown values select the first level.
/// # Examples
/// ```
/// assert_eq!(codetether_settings_policy::next_openrouter_effort(Some("high"), &["low", "high"]), None);
/// ```
/// # Panics
/// Panics if levels is empty; the caller must supply the provider level catalog.
pub fn next_openrouter_effort<'a>(current: Option<&str>, levels: &[&'a str]) -> Option<&'a str> {
    current
        .and_then(|current| levels.iter().position(|level| *level == current))
        .map_or(Some(levels[0]), |index| levels.get(index + 1).copied())
}
