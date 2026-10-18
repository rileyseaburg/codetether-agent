//! Help session 2 content and presentation.
use codetether_theme::Theme;
use codetether_theme::terminal::ColorSupport;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(
    lines: &mut Vec<Line<'static>>,
    color_support: ColorSupport,
    validated_theme: &Theme,
) {
    lines.push(Line::from(vec![
        Span::styled("  Colors:    ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            match color_support {
                ColorSupport::Monochrome => "Monochrome",
                ColorSupport::Ansi8 => "ANSI-8",
                ColorSupport::Ansi256 => "ANSI-256",
                ColorSupport::TrueColor => "TrueColor",
            },
            Style::default().fg(Color::White),
        ),
        Span::styled("  theme validated", Style::default().fg(Color::DarkGray)),
        Span::styled(
            if validated_theme.background.is_some() {
                " (bg set)"
            } else {
                " (bg default)"
            },
            Style::default().fg(Color::DarkGray),
        ),
    ]));
}
