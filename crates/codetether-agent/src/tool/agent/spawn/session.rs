//! Native session specialization for shared isolated-child construction.

codetether_agent_session::declare_isolated_session!(pub(super), crate::session::Session);

#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
