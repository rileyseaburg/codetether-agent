//! Help commands content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::separator::separator;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    // ── Slash commands ──
    lines.push(heading("SLASH COMMANDS"));
    lines.push(separator());
    lines.push(Line::from(Span::styled(
        "  Aliases: type a prefix and Tab to autocomplete",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(blank());
}
