use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use super::status_bar::bus_status_badge_span;
use crate::tui::app::state::App;

pub fn render_sessions_view(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let filter_display = if app.state.session_filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", app.state.session_filter)
    };

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw(" Session picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(app.state.status.clone()).dim(),
    ])])
    .block(Block::default().borders(Borders::ALL).title(format!(
        " Sessions (↑↓ navigate, Enter load, Esc cancel){} ",
        filter_display
    )));
    f.render_widget(header, chunks[0]);

    let filtered = app.state.filtered_sessions();
    let items: Vec<ListItem<'static>> = if filtered.is_empty() {
        if app.state.session_filter.is_empty() {
            vec![ListItem::new("No workspace sessions found")]
        } else {
            vec![ListItem::new(format!(
                "No sessions matching '{}'",
                app.state.session_filter
            ))]
        }
    } else {
        filtered
            .iter()
            .map(|(_, session)| {
                let title = session
                    .title
                    .clone()
                    .unwrap_or_else(|| "Untitled session".to_string());
                let active_marker = if app.state.session_id.as_deref() == Some(session.id.as_str())
                {
                    " ●"
                } else {
                    ""
                };
                let summary = format!(
                    "{}{}  •  {} msgs  •  {}",
                    title,
                    active_marker,
                    session.message_count,
                    session.updated_at.format("%Y-%m-%d %H:%M")
                );
                ListItem::new(summary)
            })
            .collect()
    };

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.state.selected_session.min(filtered.len() - 1)));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Sessions "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[1], &mut state);

    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            " SESSION PICKER ",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Nav "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": Load "),
        Span::styled("Type", Style::default().fg(Color::Yellow)),
        Span::raw(": Filter "),
        Span::styled("Backspace", Style::default().fg(Color::Yellow)),
        Span::raw(": Edit "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Cancel | "),
        bus_status_badge_span(app),
    ]));
    f.render_widget(help, chunks[2]);
}
