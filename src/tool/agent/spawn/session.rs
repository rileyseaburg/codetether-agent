//! Native session specialization for shared isolated-child construction.
use super::super::{params::Params, spawn_request::SpawnRequest};
use crate::session::Session;
use codetether_child_workspace::Handoff;
pub(super) async fn create(
    params: &Params,
    request: &SpawnRequest<'_>,
) -> anyhow::Result<(Session, Handoff)> {
    codetether_agent_session::create_isolated::<Session>(params, request).await
}
#[cfg(test)]
#[path = "session_tests.rs"]
mod tests;
