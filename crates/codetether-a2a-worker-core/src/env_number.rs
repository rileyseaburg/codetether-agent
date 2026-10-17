//! Small worker environment and string helpers.

/// Read an unsigned environment setting, falling back on absence or parse failure.
///
/// # Arguments
/// * `name` - Environment variable name.
/// * `default` - Value used when the variable is absent or invalid.
/// # Returns
/// Parsed unsigned value or the supplied default.
/// # Examples
/// ```rust,no_run
/// use codetether_a2a_worker_core::env_u64;
/// let timeout = env_u64("CODETETHER_WORKER_TOOL_TIMEOUT_SECS", 120);
/// ```
pub fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default)
}
