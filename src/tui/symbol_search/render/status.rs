use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::symbol_search::{SymbolSearchMode, SymbolSearchState};

pub fn render(frame: &mut Frame, state: &SymbolSearchState, area: Rect) {
    let text = if state.loading {
        Span::styled(" Searching...", Style::default().fg(Color::Yellow))
    } else if let Some(error) = &state.error {
        Span::styled(format!(" Error: {error}"), Style::default().fg(Color::Red))
    } else if state.results.is_empty() && !state.query.is_empty() {
        Span::styled(" No symbols found", Style::default().fg(Color::DarkGray))
    } else {
        let action = match state.mode {
            SymbolSearchMode::Navigate => "open",
            SymbolSearchMode::Mention { .. } => "attach",
        };
        Span::styled(
            format!(" ↑↓: navigate  Enter: {action}  Esc: close"),
            Style::default().fg(Color::DarkGray),
        )
    };
    frame.render_widget(Paragraph::new(Line::from(text)), area);
}
