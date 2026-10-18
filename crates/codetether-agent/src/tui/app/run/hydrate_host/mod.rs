//! Native metadata and UI bindings for shared startup hydration.
use crate::{session::Session, tui::app::state::App};
mod effects;
mod fields;
mod session;
pub(super) struct Native<'a>(pub &'a mut App);
