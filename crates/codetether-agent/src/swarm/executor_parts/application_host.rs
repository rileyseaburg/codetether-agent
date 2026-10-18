//! Native stage services with the application's registry and coordinator agent.
pub type Native = codetether_native_swarm::ExecutorHost<
    crate::tool::registry_lifecycle::Native,
    crate::agent::Agent,
>;
