//! Application adapter for composer attachment counts.
use crate::tui::app::state::App;
/// Build the attachment suffix from pending images.
pub fn attachment_suffix(app: &App) -> String {
    codetether_composer_view::attachment_suffix(app.state.pending_images.len())
}
