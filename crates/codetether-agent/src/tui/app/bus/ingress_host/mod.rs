//! Native bus ingress state and side-effect bindings.
use crate::tui::app::state::App;
mod effects;
mod state;
pub(super) struct Native<'a>(pub &'a mut App);
