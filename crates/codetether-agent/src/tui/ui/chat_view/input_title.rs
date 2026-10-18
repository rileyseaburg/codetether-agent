//! Application adapter for composer title labels.
use crate::tui::app::state::App;
pub(crate) fn build(app: &App, suffix: &str, width: u16) -> String {
    codetether_composer_view::build_title(&super::composer_view::projection(app), suffix, width)
}
