//! Native session bindings for shared read-only agent views.
#[cfg(test)]
use super::store;
codetether_agent_view::declare_application_view!(
    pub(in crate::tool::agent);
    crate::session::Session, crate::tool::agent::store,
);
#[cfg(test)]
#[path = "bridge_test_fixture.rs"]
mod test_fixture;
#[cfg(test)]
pub(crate) use test_fixture::record_remote_turn;
#[cfg(test)]
#[path = "bridge_transcript_tests.rs"]
mod transcript_tests;

#[cfg(test)]
#[path = "bridge_remote_tests.rs"]
mod remote_tests;
#[cfg(test)]
#[path = "bridge_tests.rs"]
mod tests;
