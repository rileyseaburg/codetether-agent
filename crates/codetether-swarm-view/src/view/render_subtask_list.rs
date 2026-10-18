//! Swarm render subtask list presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, List};
pub(super) fn render_subtask_list(f: &mut Frame, state: &mut SwarmViewState, area: Rect) {
    // Sync ListState selection from selected_index
    state.list_state.select(Some(state.selected_index));

    let items = super::swarm_view_list::subtask_items(state);

    let title = if state.subtasks.is_empty() {
        " SubTasks (none yet) "
    } else {
        " SubTasks (↑↓:select  Enter:detail) "
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(super::swarm_view_list::highlight_style())
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, area, &mut state.list_state);
}
