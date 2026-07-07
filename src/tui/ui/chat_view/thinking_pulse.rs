//! Animated thinking-pulse glyph for the streaming state.
//!
//! Cycles `◌ ◎ ◉ ◎ ◌` at 200 ms steps so the dot visibly "breathes"
//! while the assistant is generating — distinct from the braille spinner
//! and much calmer, signalling "thinking" rather than "spinning".

use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

use crate::tui::ui::chat_view::spinner::spinner_color;

const PULSE: [&str; 5] = ["◌", "◎", "◉", "◎", "◌"];

/// Current pulse glyph based on wall-clock time (200 ms steps).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::streaming_header::thinking_pulse::pulse_frame;
/// assert!(pulse_frame().chars().count() >= 1);
/// ```
pub fn pulse_frame() -> &'static str {
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 200) as usize
        % PULSE.len();
    PULSE[idx]
}

/// Build a `◌/◎/◉  thinking…` span using the live neon hue.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::streaming_header::thinking_pulse::thinking_span;
/// let _s = thinking_span();
/// ```
pub fn thinking_span() -> Span<'static> {
    let color = spinner_color();
    Span::styled(
        format!("{}  thinking…", pulse_frame()),
        Style::default()
            .fg(color)
            .add_modifier(Modifier::ITALIC | Modifier::DIM),
    )
}
