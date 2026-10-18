//! Help cmd row content and presentation.
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
pub(crate) fn cmd_row(cmd: &str, alias: &str, desc: &str) -> Line<'static> {
    let mut spans = vec![Span::styled(
        format!("  {cmd:<14}"),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )];
    if !alias.is_empty() {
        spans.push(Span::styled(
            format!("{alias:<6} "),
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        spans.push(Span::raw("       ".to_string()));
    }
    spans.push(Span::raw(desc.to_string()));
    Line::from(spans)
}
