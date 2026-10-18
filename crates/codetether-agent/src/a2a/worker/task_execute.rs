//! Application type selection for native claimed-session preparation and execution.
pub(super) type Factory = codetether_native_worker::claimed::ApplicationFactory<
    super::task_session::Loader,
    crate::tool::registry_lifecycle::Native,
    crate::a2a::worker_tool_registry::Native,
    crate::swarm::SwarmExecutor,
>;
