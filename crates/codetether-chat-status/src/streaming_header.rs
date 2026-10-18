//! Streaming separator and live assistant header.
use crate::format_timestamp;
use codetether_tui_style::{
    gradient::rgb_supported,
    gradient_rule::gradient_rule,
    spinner::{current_spinner_frame, spinner_color},
};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Build the streaming separator and assistant header.
///
/// # Arguments
/// * `elapsed` - Called after the timestamp, spinner and thinking indicator.
/// * `separator_width` - Requested separator width, capped at sixty.
///
/// # Returns
/// A separator line and header line.
///
/// # Examples
/// ```
/// let (_separator, _header) = codetether_chat_status::streaming_header(|| "elapsed".into(), 60);
/// ```
pub fn streaming_header(
    elapsed: impl FnOnce() -> Span<'static>,
    separator_width: usize,
) -> (Line<'static>, Line<'static>) {
    let neon = spinner_color();
    let width = separator_width.min(60);
    let sep = if let (true, ratatui::style::Color::Rgb(r, g, b)) = (rgb_supported(), neon) {
        gradient_rule("━", width, (r, g, b), (40, 40, 50))
    } else {
        Line::from(Span::styled("━".repeat(width), Style::default().fg(neon)))
    };
    let header = Line::from(vec![
        Span::styled(
            format!("[{}] ", format_timestamp(std::time::SystemTime::now())),
            Style::default().fg(Color::DarkGray).dim(),
        ),
        Span::styled(
            format!("{} ", current_spinner_frame()),
            Style::default().fg(neon).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "assistant ",
            Style::default().fg(neon).add_modifier(Modifier::BOLD),
        ),
        codetether_tui_style::thinking_pulse::thinking_span(),
        elapsed(),
    ]);
    (sep, header)
}
