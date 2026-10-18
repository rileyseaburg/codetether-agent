//! Maximum voice recording duration selected from the application environment.
/// Maximum recording duration from env (default 60s).
/// # Returns
/// The configured duration, capped at 300 seconds; invalid values use 60.
/// # Examples
/// ```
/// assert!(codetether_voice_capture::max_duration() <= 300);
/// ```
pub fn max_duration() -> u64 {
    std::env::var("CODETETHER_VOICE_INPUT_MAX_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(60)
        .min(300)
}
