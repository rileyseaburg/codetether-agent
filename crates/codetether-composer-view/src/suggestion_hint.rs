//! Usage-only state of the autocomplete panel.
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
pub(super) fn render(f: &mut Frame, area: Rect, hint: &str, list_state: &mut ListState) {
    let style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM);
    let item = ListItem::new(Line::from(vec![
        Span::raw("  "),
        Span::styled(hint.to_string(), style),
    ]));
    let list = List::new(vec![item]).block(Block::default().borders(Borders::ALL).title(" Usage "));
    f.render_stateful_widget(list, area, list_state);
}
