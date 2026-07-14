use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::symbol_search::{SymbolSearchMode, SymbolSearchState};

pub fn render(frame: &mut Frame, state: &SymbolSearchState, area: Rect) {
    let action = match state.mode {
        SymbolSearchMode::Navigate => "open",
        SymbolSearchMode::Mention { .. } => "attach",
    };
    let input = Paragraph::new(Line::from(vec![
        Span::styled("@ ", Style::default().fg(Color::Cyan)),
        Span::styled(&state.query, Style::default().fg(Color::Yellow)),
        Span::raw("▏"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Symbol Search (Enter: {action}, Esc: close) "))
            .border_style(Style::default().fg(Color::Cyan)),
    );
    frame.render_widget(input, area);
}
