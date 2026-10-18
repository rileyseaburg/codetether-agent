//! Native storage and view bindings for shared backward pagination.
mod view;
use crate::tui::app::state::{App, AppState};
pub(crate) type HistoryPageState = codetether_history_page::interactive::HistoryPageState<
    codetether_history_page::store::Source<crate::session::Session>,
>;
/// Apply one completed page through a fresh view of the current frontend state.
pub(crate) fn drain(app: &mut App) -> bool {
    codetether_history_page::interactive::drain(&mut view::borrow(&mut app.state))
}
/// Restore the pending history anchor using current render measurements.
pub(crate) fn apply_pending_anchor(state: &mut AppState, total_lines: usize) {
    codetether_history_page::interactive::apply_pending_history_anchor(
        &mut view::borrow(state),
        total_lines,
    );
}
