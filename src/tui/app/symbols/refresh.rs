//! Application-state adapter for shared symbol queries.
use crate::tui::app::state::App;
pub fn active(app: &App) -> bool {
    codetether_symbol_runtime::active(&app.state.symbol_search)
}
pub fn schedule(app: &mut App) {
    codetether_symbol_runtime::schedule::<crate::tool::lsp::ApplicationHost>(
        &mut app.state.symbol_search,
    );
}
pub fn drain(app: &mut App) -> bool {
    codetether_symbol_runtime::drain(&mut app.state.symbol_search)
}
pub fn close(app: &mut App) {
    codetether_symbol_runtime::close(&mut app.state.symbol_search);
}
