//! Chat-turn latency status badge.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::format_duration_ms;

/// Render completed-turn latency while idle.
///
/// # Arguments
/// * `processing` - Whether a turn is active.
/// * `last` - Lazy most-recent end-to-end latency.
/// * `average` - Lazy average, used only when a completed turn exists.
///
/// # Returns
/// A latency badge when idle with a completed turn.
///
/// # Examples
/// ```
/// assert!(codetether_chat_status::turn_latency_badge(false, || Some(100), || Some(90)).is_some());
/// ```
pub fn turn_latency_badge(
    processing: bool,
    last: impl FnOnce() -> Option<u64>,
    average: impl FnOnce() -> Option<u64>,
) -> Option<Span<'static>> {
    if processing {
        return None;
    }
    let last = last()?;
    let avg = average()
        .map(format_duration_ms)
        .unwrap_or_else(|| "—".to_string());
    Some(Span::styled(
        format!(" TURN {} (avg {})", format_duration_ms(last), avg),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::DIM),
    ))
}
