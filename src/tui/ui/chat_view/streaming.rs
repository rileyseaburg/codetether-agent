//! In-flight streaming assistant preview.
//!
//! Renders partial text via full [`MessageFormatter`].

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::app::state::AppState;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::status_bar::format_timestamp;

/// Append a streaming preview block when the app is actively receiving text.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::streaming::push_streaming_preview;
/// # fn d(s:&codetether_agent::tui::app::state::AppState){ let f=codetether_agent::tui::message_formatter::MessageFormatter::new(76); let mut l:Vec<ratatui::text::Line>=vec![]; push_streaming_preview(&mut l,s,40,&f); }
/// ```
pub fn push_streaming_preview(
    lines: &mut Vec<Line<'static>>,
    state: &AppState,
    separator_width: usize,
    formatter: &MessageFormatter,
) {
    if !state.processing || state.streaming_text.is_empty() {
        return;
    }
    lines.push(Line::from(Span::styled(
        "─".repeat(separator_width.min(40)),
        Style::default().fg(Color::DarkGray).dim(),
    )));
    lines.push(Line::from(vec![
        Span::styled(
            format!("[{}] ", format_timestamp(std::time::SystemTime::now())),
            Style::default().fg(Color::DarkGray).dim(),
        ),
        Span::styled("◆ ", Style::default().fg(Color::Cyan).bold()),
        Span::styled("assistant", Style::default().fg(Color::Cyan).bold()),
        Span::styled(
            " (streaming…)",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
        ),
    ]));
    let formatted = formatter.format_content(&state.streaming_text, "assistant");
    for line in formatted {
        let mut spans = vec![Span::styled("  ", Style::default().fg(Color::Cyan))];
        spans.extend(line.spans);
        lines.push(Line::from(spans));
    }
}
