//! Timestamp and compact duration formatting.
use std::time::{SystemTime, UNIX_EPOCH};

/// Format a wall-clock time within its UTC day.
///
/// # Arguments
/// * `timestamp` - Wall-clock time to format.
///
/// # Returns
/// Hours, minutes and seconds as HH:MM:SS.
///
/// # Examples
/// ```
/// assert_eq!(codetether_chat_status::format_timestamp(std::time::UNIX_EPOCH), "00:00:00");
/// ```
pub fn format_timestamp(timestamp: SystemTime) -> String {
    let secs = timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let day = secs % 86_400;
    let hour = day / 3_600;
    let minute = (day % 3_600) / 60;
    let second = day % 60;
    format!("{hour:02}:{minute:02}:{second:02}")
}

/// Format milliseconds as a compact duration.
///
/// # Arguments
/// * `duration_ms` - Duration in milliseconds.
///
/// # Returns
/// Milliseconds, seconds or minutes depending on magnitude.
///
/// # Examples
/// ```
/// assert_eq!(codetether_chat_status::format_duration_ms(1500), "1.5s");
/// ```
pub fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 60_000 {
        format!(
            "{}m{:02}s",
            duration_ms / 60_000,
            (duration_ms % 60_000) / 1_000
        )
    } else if duration_ms >= 1_000 {
        format!("{:.1}s", duration_ms as f64 / 1_000.0)
    } else {
        format!("{duration_ms}ms")
    }
}
