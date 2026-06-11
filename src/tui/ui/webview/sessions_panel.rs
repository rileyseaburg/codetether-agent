use ratatui::{prelude::*, widgets::*};

use crate::{session::SessionSummary, tui::app::state::App};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    let visible = area.height.saturating_sub(2) as usize / 2;
    for (idx, summary) in app.state.sessions.iter().take(visible).enumerate() {
        lines.push(title_line(idx, app.state.selected_session, summary));
        lines.push(Line::styled(
            meta(summary),
            Style::default().fg(Color::DarkGray),
        ));
    }
    if lines.is_empty() {
        lines.push(Line::styled(
            "No recent sessions",
            Style::default().fg(Color::DarkGray),
        ));
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Recent Sessions ");
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn title_line(idx: usize, selected: usize, summary: &SessionSummary) -> Line<'static> {
    let marker = if idx == selected { "●" } else { "○" };
    let style = if idx == selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::Gray)
    };
    Line::from(vec![Span::styled(
        format!(
            "{marker} {}",
            crate::util::truncate_bytes_safe(summary.title.as_deref().unwrap_or("Untitled"), 22)
        ),
        style,
    )])
}

fn meta(summary: &SessionSummary) -> String {
    format!(
        "{} msgs • {}",
        summary.message_count,
        summary.updated_at.format("%m-%d %H:%M")
    )
}
