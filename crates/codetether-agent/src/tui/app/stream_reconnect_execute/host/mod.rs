//! Native bindings for same-model reconnect execution.
mod runtime;
mod ui;
use super::{App, Arc, ProviderRegistry, SessionSlot, TuiSessionHandle};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub runtime: &'a TuiSessionHandle,
}
