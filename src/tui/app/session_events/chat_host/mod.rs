//! Native state bindings for shared chat event workflows.
use crate::tui::app::state::App;
mod stream;
mod usage;
pub(super) struct Native<'a>(pub &'a mut App);
