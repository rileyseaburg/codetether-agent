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
    status_line.spans.insert(
        0,
        Span::styled(model_status, Style::default().fg(Color::Cyan)),
    );
    status_line
        .spans
        .insert(0, Span::styled("│ ", Style::default().fg(Color::DarkGray)));
    let para = Paragraph::new(status_line);
    f.render_widget(para, area);
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
