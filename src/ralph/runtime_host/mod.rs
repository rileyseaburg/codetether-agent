//! Native type choices for the shared Ralph application runtime.
mod relay;
/// Runtime binding retaining the native provider, tool, agent and relay identities.
pub type Native = codetether_ralph_runtime::application::ApplicationHost<
    crate::provider::ProviderRegistry,
    crate::tool::registry_lifecycle::Native,
    codetether_ralph_agent::application::ApplicationAgent<
        crate::tool::registry_lifecycle::Native,
        crate::swarm::executor::LoopHost,
    >,
    relay::Native,
>;
