use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::App;

/// Minimum terminal size for webview layout.
const MIN_WIDTH: u16 = 90;
const MIN_HEIGHT: u16 = 18;

pub fn render_webview_chat_center(f: &mut Frame, app: &App, area: Rect, lines: &[Line<'static>]) {
    let visible = area.height.saturating_sub(2) as usize;
    let scroll = app
        .state
        .chat_scroll
        .min(lines.len().saturating_sub(visible));
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::default()));
    let para = Paragraph::new(lines.to_vec())
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(para, area);
}

pub fn render_webview_input(f: &mut Frame, app: &App, area: Rect) {
    let mode_label = if app.state.input_mode == crate::tui::models::InputMode::MultiLine {
        " [MULTI]"
    } else {
        ""
    };
    let title = format!(" Input{mode_label} ");
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));
    let para = Paragraph::new(app.state.input.as_str())
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

pub fn terminal_too_small(area: Rect) -> bool {
    area.width < MIN_WIDTH || area.height < MIN_HEIGHT
}
