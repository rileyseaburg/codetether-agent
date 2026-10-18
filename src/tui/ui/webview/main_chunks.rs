//! Native composer projection for shared Webview row geometry.
use crate::tui::app::state::App;
pub(super) use codetether_chat_layout::webview::MainChunks;
use ratatui::layout::Rect;
pub(super) fn compute(area: Rect, app: &App) -> MainChunks {
    codetether_chat_layout::webview::compute(
        area,
        &app.state.input,
        app.state.slash_suggestions_visible(),
    )
}
