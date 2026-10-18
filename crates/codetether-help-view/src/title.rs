//! Help title content and presentation.
use crate::blank::blank;
use crate::separator::separator;
use codetether_token_display::TokenDisplay;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>, token_display: &TokenDisplay) {
    // ── Title ──
    lines.push(Line::from(vec![
        Span::styled(
            "  CodeTether TUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  ·  Help & Reference",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(separator());
    lines.push(blank());
    for line in token_display.create_detailed_display().into_iter().take(8) {
        lines.push(Line::from(line));
    }
    lines.push(blank());
}
