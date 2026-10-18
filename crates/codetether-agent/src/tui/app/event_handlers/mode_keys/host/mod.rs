//! Native bindings for mode-specific input selection.
mod input;
mod state;
use super::{App, SessionSlot};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
}
