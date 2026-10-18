//! Fixed status-bar navigation hints.
use codetether_theme::Theme;
use ratatui::{style::Style, text::Span};
pub(crate) fn append(spans: &mut Vec<Span<'static>>, theme: &Theme) {
    // Help indicator
    spans.push(Span::styled(
        " ? ",
        Style::default()
            .fg(theme.status_bar_foreground.to_color())
            .bg(theme.status_bar_background.to_color()),
    ));
    spans.push(Span::raw(" Help "));

    // Switch agent
    spans.push(Span::styled(
        " Tab ",
        Style::default()
            .fg(theme.status_bar_foreground.to_color())
            .bg(theme.status_bar_background.to_color()),
    ));
    spans.push(Span::raw(" Switch Agent "));

    // Quit
    spans.push(Span::styled(
        " Ctrl+C ",
        Style::default()
            .fg(theme.status_bar_foreground.to_color())
            .bg(theme.status_bar_background.to_color()),
    ));
    spans.push(Span::raw(" Quit "));
}
