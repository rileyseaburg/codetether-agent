//! Native application bindings for shared Webview composition.
mod composer;
mod overlays;
mod panels;
use crate::tui::app::state::App;
pub(super) struct Native<'a>(pub &'a mut App);
impl codetether_webview::State for Native<'_> {
    fn input(&self) -> &str {
        &self.0.state.composer.input
    }
    fn suggestions_visible(&self) -> bool {
        self.0.state.slash_suggestions_visible()
    }
}
