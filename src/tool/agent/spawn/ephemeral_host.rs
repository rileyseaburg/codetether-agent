//! Native one-shot runtime with the application's tool catalog and step limit.
pub type ApplicationHost = codetether_native_agent::Ephemeral<
    crate::tool::registry_lifecycle::Native,
    { crate::session::DEFAULT_MAX_STEPS },
>;
