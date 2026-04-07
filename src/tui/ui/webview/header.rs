use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::App;

pub fn render_webview_header(f: &mut Frame, app: &App, area: Rect) {
    let model_label = app.state.last_completion_model.as_deref().unwrap_or("auto");
    let title = format!(" CodeTether ─ {model_label} ");
    let mode_label = if cfg!(debug_assertions) {
        " [DEBUG]"
    } else {
        ""
    };
    let right = format!("Chat{mode_label} ");
    let title_len = title.len() as u16;
    let right_len = right.len() as u16;
    let spacing = area.width.saturating_sub(title_len).saturating_sub(right_len);
    let line = Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" ".repeat(spacing as usize)),
        Span::styled(right, Style::default().fg(Color::DarkGray)),
    ]);
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(line).block(block);
    f.render_widget(para, area);
}
