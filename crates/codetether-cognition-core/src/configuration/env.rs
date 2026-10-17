//! Typed environment-variable readers with defaults.

/// Read a boolean flag, accepting `1/true/yes/on` and `0/false/no/off`.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Fallback used when absent or unparseable.
///
/// # Returns
/// The parsed setting or assembled configuration using existing defaults.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::configuration::env::env_bool;
/// let _value = env_bool("EXAMPLE_COGNITION_SETTING", false);
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

/// Read an `f32`, returning `default` when unset or unparseable.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Fallback used when absent or unparseable.
///
/// # Returns
/// The parsed setting or assembled configuration using existing defaults.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::configuration::env::env_f32;
/// let _value = env_f32("EXAMPLE_COGNITION_SETTING", 0.0);
/// ```
pub fn env_f32(name: &str, default: f32) -> f32 {
    parse_or(name, default)
}

/// Read a `u64`, returning `default` when unset or unparseable.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Fallback used when absent or unparseable.
///
/// # Returns
/// The parsed setting or assembled configuration using existing defaults.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::configuration::env::env_u64;
/// let _value = env_u64("EXAMPLE_COGNITION_SETTING", 0);
/// ```
pub fn env_u64(name: &str, default: u64) -> u64 {
    parse_or(name, default)
}

/// Read a `u32`, returning `default` when unset or unparseable.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Fallback used when absent or unparseable.
///
/// # Returns
/// The parsed setting or assembled configuration using existing defaults.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::configuration::env::env_u32;
/// let _value = env_u32("EXAMPLE_COGNITION_SETTING", 0);
/// ```
pub fn env_u32(name: &str, default: u32) -> u32 {
    parse_or(name, default)
}

/// Read a `usize`, returning `default` when unset or unparseable.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Fallback used when absent or unparseable.
///
/// # Returns
/// The parsed setting or assembled configuration using existing defaults.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::configuration::env::env_usize;
/// let _value = env_usize("EXAMPLE_COGNITION_SETTING", 0);
/// ```
pub fn env_usize(name: &str, default: usize) -> usize {
    parse_or(name, default)
}

fn parse_or<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}
