//! Application bindings for shared managed-child session lifecycle rules.
use crate::session::Session;
use std::path::PathBuf;
#[path = "session_factory/binding.rs"]
mod binding;
#[cfg(test)]
use codetether_agent_messages::system_message as build_system_message;
pub(super) async fn create_agent_session(
    name: &str,
    instructions: &str,
    model: &str,
    parent_workspace: Option<PathBuf>,
    prior_context_allowed: bool,
) -> anyhow::Result<Session> {
    codetether_agent_session::create::<Session>(
        name,
        instructions,
        model,
        parent_workspace,
        prior_context_allowed,
    )
    .await
}
pub(super) async fn parent_prior_context_allowed(
    inherited: Option<bool>,
    parent_id: Option<&str>,
) -> bool {
    codetether_agent_session::parent_prior_context_allowed::<Session>(inherited, parent_id).await
}
#[cfg(test)]
#[path = "session_factory/policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
mod tests;
