//! Session picker item construction for visible rows.

use std::ops::Range;

use ratatui::widgets::ListItem;

use crate::PickerView;
use crate::session_list_item;
use codetether_session_metadata::SessionSummary;

pub(super) fn items(
    view: &PickerView<'_>,
    filtered: &[(usize, &SessionSummary)],
    range: Range<usize>,
) -> Vec<ListItem<'static>> {
    if filtered.is_empty() {
        return vec![ListItem::new(empty_message(view))];
    }
    filtered[range]
        .iter()
        .map(|(_, session)| {
            let active = view.active_session == Some(session.id.as_str());
            session_list_item(session, active)
        })
        .collect()
}

fn empty_message(view: &PickerView<'_>) -> String {
    if view.filter.is_empty() {
        "No workspace sessions found".to_string()
    } else {
        format!("No sessions matching '{}'", view.filter)
    }
}
