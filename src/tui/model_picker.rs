use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::session::Session;
use crate::tui::app::state::App;

pub fn render_model_picker(f: &mut Frame, area: Rect, app: &mut App, session: &Session) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let filter_suffix = if app.state.model_filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", app.state.model_filter)
    };

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw(" Model picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(app.state.status.clone()).dim(),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Models{} ", filter_suffix)),
    );
    f.render_widget(header, chunks[0]);

    let models = app.state.filtered_models();
    let items: Vec<ListItem<'static>> = if models.is_empty() {
        let empty_label = if app.state.model_refresh_in_flight {
            "Loading models..."
        } else {
            "No models available"
        };
        vec![ListItem::new(empty_label)]
    } else {
        models
            .iter()
            .map(|model| ListItem::new((*model).to_string()))
            .collect()
    };

    let mut list_state = ListState::default();
    if !models.is_empty() {
        list_state.select(Some(app.state.selected_model_index.min(models.len() - 1)));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Models "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[1], &mut list_state);

    let current = app
        .state
        .selected_model()
        .or(session.metadata.model.as_deref())
        .unwrap_or("auto");
    let current_widget = Paragraph::new(Line::from(vec![
        Span::raw("Current selection: "),
        Span::styled(current.to_string(), Style::default().fg(Color::Cyan).bold()),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Selection "));
    f.render_widget(current_widget, chunks[2]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" MODEL ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Nav "),
        Span::styled("Type", Style::default().fg(Color::Yellow)),
        Span::raw(": Filter "),
        Span::styled("Backspace", Style::default().fg(Color::Yellow)),
        Span::raw(": Edit "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": Apply "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Cancel | "),
        Span::raw(app.state.status.clone()).dim(),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Controls "));
    f.render_widget(footer, chunks[3]);
}
