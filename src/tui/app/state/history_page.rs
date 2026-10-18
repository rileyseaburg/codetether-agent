//! Native storage and view bindings for shared backward pagination.
#[path = "history_page/view.rs"]
mod view;
use crate::tui::app::state::{App, AppState};
pub(crate) type HistoryPageState = codetether_history_page::interactive::HistoryPageState<
    codetether_history_page::store::Source<crate::session::Session>,
>;
/// Apply one completed page, if available.
pub(crate) fn drain(app: &mut App) -> bool {
    codetether_history_page::interactive::drain(&mut app.state)
}
impl AppState {
    /// Restore the viewport anchor after older messages were prepended.
    pub(crate) fn apply_pending_history_anchor(&mut self, total_lines: usize) {
        codetether_history_page::interactive::apply_pending_history_anchor(self, total_lines);
    }
}
