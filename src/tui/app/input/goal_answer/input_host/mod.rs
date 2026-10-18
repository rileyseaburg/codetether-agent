//! Session-view and UI bindings for shared question interception.
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
