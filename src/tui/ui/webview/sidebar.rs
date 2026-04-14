use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::tui::app::state::App;

pub fn render_webview_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .title(" Sessions ")
        .border_style(Style::default().fg(Color::DarkGray));
    let mut items: Vec<ListItem> = Vec::new();
    for (i, summary) in app.state.sessions.iter().enumerate() {
        let title = summary.title.as_deref().unwrap_or("Untitled");
        let label = if i == app.state.selected_session {
            format!("▶ {}", truncate_str(title, 20))
        } else {
            format!("  {}", truncate_str(title, 20))
        };
        let style = if i == app.state.selected_session {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        items.push(ListItem::new(Line::from(Span::styled(label, style))));
    }
    if items.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "  No sessions",
            Style::default().fg(Color::DarkGray),
        ))));
    }
    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn truncate_str(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let boundary = s.floor_char_boundary(max.saturating_sub(1));
        format!("{}…", &s[..boundary])
    }
}
