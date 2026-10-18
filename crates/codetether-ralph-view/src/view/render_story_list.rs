//! Ralph render story list presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};
pub(super) fn render_story_list(f: &mut Frame, state: &mut RalphViewState, area: Rect) {
    state.list_state.select(Some(state.selected_index));

    let items: Vec<ListItem> = state.stories.iter().map(super::story_item::item).collect();

    let title = if state.stories.is_empty() {
        " Stories (loading...) "
    } else {
        " Stories (↑↓:select  Enter:detail) "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut state.list_state);
}
