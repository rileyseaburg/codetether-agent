//! Direct native state bindings for shared agent command behavior.
use crate::tui::app::state::App;
mod listing;
mod registry;
mod state;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub parent: Option<&'a str>,
}

mod selection;
