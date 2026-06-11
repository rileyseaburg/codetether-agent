use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::App;

pub fn render_webview_header(f: &mut Frame, app: &App, area: Rect) {
    let line = Line::from(header_text(app, area.width));
    let block = Block::default()
        .title(" CodeTether Webview ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let para = Paragraph::new(line)
        .block(block)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(para, area);
}

fn header_text(app: &App, width: u16) -> String {
    let session = app.state.session_id.as_deref().unwrap_or("#new");
    let agent = app.state.active_spawned_agent.as_deref().unwrap_or("main");
    let branch = app
        .state
        .workspace
        .git_branch
        .as_deref()
        .unwrap_or("no git");
    let text = format!(
        "Workspace Chat {session} | {agent} | {branch} | {} msgs",
        app.state.messages.len()
    );
    crate::util::truncate_bytes_safe(&text, width.saturating_sub(4) as usize).to_string()
}
