//! Session picker item construction for visible rows.

use std::ops::Range;

use ratatui::widgets::ListItem;

use crate::session::SessionSummary;
use crate::tui::app::state::App;
use crate::tui::ui::sessions_list_item::session_list_item;

pub(super) fn items(
    app: &App,
    filtered: &[(usize, &SessionSummary)],
    range: Range<usize>,
) -> Vec<ListItem<'static>> {
    if filtered.is_empty() {
        return vec![ListItem::new(empty_message(app))];
    }
    filtered[range]
        .iter()
        .map(|(_, session)| {
            let active = app.state.session_id.as_deref() == Some(session.id.as_str());
            session_list_item(session, active)
        })
        .collect()
}

fn empty_message(app: &App) -> String {
    if app.state.session_filter.is_empty() {
        "No workspace sessions found".to_string()
    } else {
        format!("No sessions matching '{}'", app.state.session_filter)
    }
}
