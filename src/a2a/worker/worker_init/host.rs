//! Application catalog and peer hooks for the shared native initializer.
use codetether_a2a_http::peer_handle::A2APeerHandle;
use codetether_bus_core::{AgentBus, BusHandle};
use codetether_cli_options::A2aArgs;
use std::sync::Arc;
pub(in crate::a2a::worker) struct Native;
impl codetether_native_worker::initialization::Hooks for Native {
    fn announce(handle: &BusHandle) -> usize {
        codetether_native_registry::announce::<crate::tool::registry_lifecycle::Catalog>(handle)
    }
    async fn peer(args: &A2aArgs, name: &str, bus: Arc<AgentBus>) -> Option<A2APeerHandle> {
        super::super::maybe_start_worker_a2a_peer(args, name, bus).await
    }
}
