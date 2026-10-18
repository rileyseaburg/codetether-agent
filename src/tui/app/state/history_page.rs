//! Native storage and view bindings for shared backward pagination.
use crate::tui::app::state::{App, AppState};
pub(crate) type HistoryPageState = codetether_history_page::interactive::HistoryPageState<
    codetether_history_page::store::Source<crate::session::Session>,
>;
codetether_history_page::impl_app_history_page!(
    App,
    AppState,
    codetether_history_page::store::Source<crate::session::Session>,
    crate::tui::app::message_cache_invalidate::clear,
    crate::tui::constants::SCROLL_BOTTOM
);
