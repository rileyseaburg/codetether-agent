//! Boolean environment settings with caller-supplied defaults.

/// Read a boolean environment value or use the supplied default.
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Value used for missing or unrecognized settings.
/// # Returns
/// Case-insensitive 1/true/yes/on or 0/false/no/off values, otherwise default.
/// # Examples
/// ```no_run
/// let _enabled = codetether_util::environment::env_bool("FEATURE_ENABLED", false);
/// ```
pub fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|v| match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}
