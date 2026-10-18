//! Native type selections for application-owned verifier execution.
pub(super) type Native = codetether_goal_verification::runtime::application::ApplicationHost<
    crate::provider::registry::application::Native,
    crate::tool::registry_lifecycle::Native,
    crate::swarm::executor::LoopHost,
    crate::swarm::executor::workspace_registry::RegistryHost,
>;
