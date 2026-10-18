//! Native view and command adapters without borrowing the active session.
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
mod actions;
mod view;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a SessionSlot,
    pub runtime: &'a TuiSessionHandle,
}
