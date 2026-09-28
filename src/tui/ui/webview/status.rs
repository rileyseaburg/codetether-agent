use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::App;
use crate::tui::theme::Theme;
use crate::tui::theme_utils::validate_theme;
use crate::tui::token_display::TokenDisplay;

pub fn render_webview_status(f: &mut Frame, app: &App, area: Rect) {
    let token_display = TokenDisplay::new();
    let validated_theme = validate_theme(&Theme::default());
    let mut status_line = token_display.create_status_bar(&validated_theme);
    let model_status = app
        .state
        .last_completion_model
        .as_deref()
        .map(|m| format!(" {m} "))
        .unwrap_or_else(|| " auto ".to_string());
    let mut prefix = super::status_prefix::build(app);
    prefix.push(Span::styled(model_status, Style::default().fg(Color::Cyan)));
    prefix.push(Span::styled("│ ", Style::default().fg(Color::DarkGray)));
    status_line.spans.splice(0..0, prefix);
    let owned: Vec<Span<'static>> = status_line
        .spans
        .into_iter()
        .map(|span| Span::styled(span.content.into_owned(), span.style))
        .collect();
    let lines = crate::tui::ui::chat_view::status_pack::pack_spans(owned, area.width);
    f.render_widget(Paragraph::new(lines), area);
}

/// Rows the status bar needs at `width` (1 on wide terminals, 2 on phones).
pub fn status_height(width: u16) -> u16 {
    if width >= 140 { 1 } else { 2 }
}

/// Fallback message shown when terminal is too small for webview.
pub fn render_too_small(f: &mut Frame, area: Rect) {
    let msg = format!(
        "Terminal too small for Webview (need {}×{}). Try resizing or /classic.",
        90, 18
    );
    let line = Line::from(msg.red());
    let para = Paragraph::new(line);
    f.render_widget(para, area);
}
