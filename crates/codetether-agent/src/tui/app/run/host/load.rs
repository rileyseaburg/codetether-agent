//! Native provider, worker identity and session loaders.
use super::*;
impl codetether_tui_startup::Load for Native {
    type Registry = ProviderRegistry;
    type Bridge = TuiWorkerBridge;
    type Tail = TailLoad;
    fn providers() -> impl std::future::Future<Output = Option<Arc<ProviderRegistry>>> {
        provider::load_registry()
    }
    fn worker(
        bus: Arc<AgentBus>,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<TuiWorkerBridge>>> {
        TuiWorkerBridge::spawn(None, None, crate::provenance::runtime_agent_identity(), bus)
    }
    fn session(id: Option<&str>) -> impl std::future::Future<Output = anyhow::Result<TailLoad>> {
        session_scan::load(id)
    }
}
