//! Same-turn and queued delivery for inter-agent messages.

codetether_agent_communication::declare_application_communication!(
    pub(crate);
    crate::session::Session,
    crate::tool::agent::store,
    crate::session::helper::steering,
);

#[cfg(test)]
#[path = "communication/active_tests.rs"]
mod active_tests;
#[cfg(test)]
#[path = "communication/idle_tests.rs"]
mod idle_tests;
