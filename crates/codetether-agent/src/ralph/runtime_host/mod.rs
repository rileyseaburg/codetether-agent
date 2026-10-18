//! Application type choices for native Ralph services.
mod relay;
/// Runtime binding retaining native tool, agent and session identities.
pub type Native = codetether_native_ralph::Host<
    crate::tool::registry_lifecycle::Native,
    crate::swarm::executor::LoopHost,
    crate::session::Session,
>;
