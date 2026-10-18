//! Compact elapsed-time labels for active operations.

/// Format the time elapsed since `started` as a human-readable string.
///
/// Returns ` MmSS` for durations ≥ 60 s, otherwise ` S.s`.
///
/// # Examples
///
/// ```rust
/// use codetether_tui_style::spinner::format_elapsed;
/// let s = format_elapsed(std::time::Instant::now());
/// assert!(s.starts_with(' '));
/// assert!(s.contains('s'));
/// ```
///
/// # Arguments
/// * `started` - Monotonic start time.
///
/// # Returns
/// An elapsed label with a leading space.
pub fn format_elapsed(started: std::time::Instant) -> String {
    let elapsed = started.elapsed();
    if elapsed.as_secs() >= 60 {
        format!(" {}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
    } else {
        format!(" {:.1}s", elapsed.as_secs_f64())
    }
}
