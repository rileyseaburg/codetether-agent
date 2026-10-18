//! Durable child loading with bounded LRU residency.

codetether_agent_residency::declare_application_residency!(
    pub(in crate::tool::agent), pub(in crate::tool),
    crate::session::Session,
    crate::tool::agent::persistence::ClosedAgent,
    crate::tool::agent::store,
    crate::tool::agent::persistence,
    crate::tool::agent::event_loop::live_trace::clear,
    codetether_agent_session::apply_overlay,
);

#[cfg(test)]
include!("residency/test_modules.rs");
