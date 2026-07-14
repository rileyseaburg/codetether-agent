mod item;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List},
};

use crate::tui::symbol_search::SymbolSearchState;

pub fn render(frame: &mut Frame, state: &mut SymbolSearchState, area: Rect) {
    let items = state.results.iter().map(item::render).collect::<Vec<_>>();
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Results ({}) ", state.results.len())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    state
        .list_state
        .select((!state.results.is_empty()).then_some(state.selected));
    frame.render_stateful_widget(list, area, &mut state.list_state);
}
