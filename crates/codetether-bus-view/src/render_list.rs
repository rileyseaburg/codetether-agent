//! Scrollable list renderer for bus log entries.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
};

use super::{render_item, state::BusLogState};

pub(super) fn render_list(f: &mut Frame, state: &mut BusLogState, area: Rect) {
    let filtered = state.filtered_entries();
    let filtered_len = filtered.len();
    let title = title(state, filtered_len);
    let items: Vec<ListItem<'static>> = filtered
        .iter()
        .enumerate()
        .map(|(idx, entry)| render_item::item(idx, entry, state.selected_index))
        .collect();
    drop(filtered);
    state.list_state.select(Some(
        state.selected_index.min(filtered_len.saturating_sub(1)),
    ));
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_stateful_widget(list, area, &mut state.list_state);
}

fn title(state: &BusLogState, filtered_len: usize) -> String {
    if state.filter.is_empty() {
        format!("Protocol Bus Log ({filtered_len})")
    } else if state.filter_input_mode {
        format!("Protocol Bus Log [{}_] ({filtered_len})", state.filter)
    } else {
        format!("Protocol Bus Log [{}] ({filtered_len})", state.filter)
    }
}
