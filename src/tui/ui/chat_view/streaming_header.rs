//! Neon header line for the in-flight streaming preview.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::app::state::AppState;
use crate::tui::ui::status_bar::format_timestamp;

use super::elapsed_badge::elapsed_badge;
use super::spinner::{current_spinner_frame, spinner_color};
use crate::tui::ui::gradient::rgb_supported;
use crate::tui::ui::gradient_rule::gradient_rule;

#[path = "thinking_pulse.rs"]
pub mod thinking_pulse;

/// Build the neon separator + header line for the streaming preview block.
///
/// Returns `(separator_line, header_line)`.
pub(super) fn streaming_header(
    state: &AppState,
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
        thinking_pulse::thinking_span(),
        elapsed_badge(state),
    ]);
    (sep, header)
}
