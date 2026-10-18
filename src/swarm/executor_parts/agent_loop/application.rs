//! Native swarm agent services with the application's concrete tool registry.
pub(crate) type Native =
    codetether_native_swarm::AgentHost<crate::tool::registry_lifecycle::Native>;
