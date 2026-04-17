//! Animated spinner and elapsed-time formatting.
//!
//! Ported from the legacy TUI. [`current_spinner_frame`] cycles through
//! braille-pattern glyphs every 100 ms. [`format_elapsed`] renders an
//! [`Instant`] delta as `MmSS` or `S.s` seconds.

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Return the current braille-spinner glyph based on wall-clock time.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::spinner::current_spinner_frame;
/// let frame = current_spinner_frame();
/// assert!(frame.chars().count() >= 1);
/// ```
pub fn current_spinner_frame() -> &'static str {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 100) as usize
        % SPINNER.len();
    SPINNER[idx]
}

/// Format the time elapsed since `started` as a human-readable string.
///
/// Returns ` MmSS` for durations ≥ 60 s, otherwise ` S.s`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::spinner::format_elapsed;
/// // Instant::now() has ≈0 s elapsed, so format is " 0.xs"
/// let s = format_elapsed(std::time::Instant::now());
/// assert!(s.starts_with(' '));
/// assert!(s.contains('s'));
/// ```
pub fn format_elapsed(started: std::time::Instant) -> String {
    let elapsed = started.elapsed();
    if elapsed.as_secs() >= 60 {
        format!(" {}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
    } else {
        format!(" {:.1}s", elapsed.as_secs_f64())
    }
}
