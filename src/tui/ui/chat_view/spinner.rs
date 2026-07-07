//! Animated spinner glyphs and neon cycling colors.
//!
//! [`current_spinner_frame`] cycles through a dense braille-arc pattern.
//! [`spinner_color`] returns a neon hue that rotates cyan → magenta → yellow
//! every ~300 ms so the spinner visually "pulses" with color.
//! [`format_elapsed`] renders an [`Instant`] delta as `MmSS` or `S.s`.

use ratatui::style::Color;

const SPINNER: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// Neon colors that cycle every ~300 ms.
const NEON: [Color; 6] = [
    Color::Cyan,
    Color::LightCyan,
    Color::Magenta,
    Color::LightMagenta,
    Color::Yellow,
    Color::LightYellow,
];

/// Return the current spinner glyph based on wall-clock time (100 ms steps).
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

/// Return a neon [`Color`] that cycles every ~300 ms.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::spinner::spinner_color;
/// let _c = spinner_color();
/// ```
pub fn spinner_color() -> Color {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 300) as usize
        % NEON.len();
    NEON[idx]
}

/// Format the time elapsed since `started` as a human-readable string.
///
/// Returns ` MmSS` for durations ≥ 60 s, otherwise ` S.s`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::spinner::format_elapsed;
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
