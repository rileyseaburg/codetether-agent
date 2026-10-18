//! Native session and runtime bindings for shared handoff sequencing.
mod preparation;
mod transfer;
use super::{App, Arc, ProviderRegistry, SessionSlot, TuiSessionHandle};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Arc<ProviderRegistry>,
    pub runtime: &'a TuiSessionHandle,
}
