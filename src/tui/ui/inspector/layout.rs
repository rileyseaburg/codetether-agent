use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::AppState;

use super::{metrics::render_metrics, tool_history::render_tool_history};

pub fn render_inspector(f: &mut Frame, state: &AppState) {
    let area = f.area();
    let outer = Block::default()
        .borders(Borders::ALL)
        .title(" Inspector ")
        .title_style(Style::default().fg(Color::Cyan));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(6)])
        .split(inner);

    render_metrics(f, state, chunks[0]);
    render_tool_history(f, state, chunks[1]);

    render_footer(f, state, area);
}

fn render_footer(f: &mut Frame, state: &AppState, area: Rect) {
    let footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let help = Paragraph::new(Line::styled(
        format!(
            " Esc: back to chat │ Model: {} ",
            state.last_completion_model.as_deref().unwrap_or("auto")
        ),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(help, footer[1]);
}
