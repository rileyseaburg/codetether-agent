//! Packed status lines and terminal-size fallback.
//! [render] combines badges with token spans; [render_too_small] displays the
//! minimum terminal dimensions when the full view cannot fit.
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

/// Combine token spans, status badges and the active model in a packed bar.
/// # Arguments
/// * `f` - Destination frame.
/// * `status_line` - Token summary supplied by the caller.
/// * `prefix` - Owned status badges.
/// * `model` - Last completion model, or automatic selection when absent.
/// * `area` - Available status rectangle.
/// # Returns
/// Draws packed status lines into the frame.
/// # Examples
/// ```no_run
/// # let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 2)).unwrap();
/// terminal.draw(|frame| {
///     let area = frame.area();
///     codetether_webview_bars::status::render(frame, ratatui::text::Line::default(), vec![], None, area);
/// }).unwrap();
/// ```
pub fn render(
    f: &mut Frame,
    mut status_line: Line<'_>,
    mut prefix: Vec<Span<'static>>,
    model: Option<&str>,
    area: Rect,
) {
    let model_status = model
        .map(|m| format!(" {m} "))
        .unwrap_or_else(|| " auto ".to_string());
    prefix.push(Span::styled(model_status, Style::default().fg(Color::Cyan)));
    prefix.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));
    status_line.spans.splice(0..0, prefix);
    let owned: Vec<Span<'static>> = status_line
        .spans
        .into_iter()
        .map(|span| Span::styled(span.content.into_owned(), span.style))
        .collect();
    let lines = codetether_chat_layout::pack_spans(owned, area.width);
    f.render_widget(Paragraph::new(lines), area);
}

/// Fallback message shown when terminal is too small for Webview.
/// # Arguments
/// * `f` - Destination frame.
/// * `area` - Available terminal rectangle.
/// # Returns
/// Draws a red terminal-size hint.
/// # Examples
/// ```no_run
/// # let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(60, 10)).unwrap();
/// terminal.draw(|frame| {
///     let area = frame.area();
///     codetether_webview_bars::status::render_too_small(frame, area);
/// }).unwrap();
/// ```
pub fn render_too_small(f: &mut Frame, area: Rect) {
    let msg = format!(
        "Terminal too small for Webview (need {}×{}). Try resizing or /classic.",
        90, 18
    );
    let line = Line::from(msg.red());
    let para = Paragraph::new(line);
    f.render_widget(para, area);
}
