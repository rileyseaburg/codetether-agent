//! Application bindings for shared managed-child session lifecycle rules.

codetether_agent_session::declare_session_factory!(
    pub(super);
    crate::session::Session,
    crate::session::helper::runtime::prior_context_allowed_for_session,
);

#[cfg(test)]
use codetether_agent_messages::system_message as build_system_message;
#[cfg(test)]
#[path = "session_factory/policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
mod tests;
