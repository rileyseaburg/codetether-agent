//! Bounded session-resume window selected from the process environment.
const DEFAULT_WINDOW: usize = 1_000;
const MAX_WINDOW: usize = 10_000;

/// Resolve the configured resume window, preserving its default and upper cap.
///
/// # Returns
/// A positive count of messages to retain when loading a session tail.
///
/// # Examples
/// ```
/// assert!((1..=10_000).contains(&codetether_session_resume::session_resume_window()));
/// ```
pub fn session_resume_window() -> usize {
    let parsed = std::env::var("CODETETHER_SESSION_RESUME_WINDOW")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0);
    match parsed {
        Some(value) if value > MAX_WINDOW => {
            tracing::warn!(
                requested = value,
                clamped = MAX_WINDOW,
                "session resume window too large; clamping"
            );
            MAX_WINDOW
        }
        Some(value) => value,
        None => DEFAULT_WINDOW,
    }
}
