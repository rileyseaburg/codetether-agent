//! Spawn orchestration for the agent tool.
//!
//! Coordinates spawn request parsing, validation, session creation, and the
//! durable/ephemeral persistence policy. After persistence succeeds, the first
//! model turn is auto-started via [`super::spawn_run`] so the sub-agent begins
//! working immediately instead of sitting idle (issue #295).

#[path = "spawn/ephemeral.rs"]
mod ephemeral;
#[path = "spawn/ephemeral_host.rs"]
mod ephemeral_host;
#[cfg(test)]
#[path = "spawn/ephemeral_task.rs"]
mod ephemeral_task;
#[path = "spawn/identity.rs"]
mod identity;
#[path = "spawn/session.rs"]
mod session;
#[path = "spawn/workspace.rs"]
mod workspace;
codetether_agent_spawn::declare_application_spawn!(
    pub(super),
    crate::session::Session,
    crate::tool::agent::residency::reserve,
    crate::tool::agent::spawn::session::create,
    crate::tool::agent::spawn_store::persist_spawned_agent,
    crate::tool::agent::spawn_validation::validate_spawn_request,
    crate::tool::agent::spawn::identity::reserve,
    crate::tool::agent::spawn_run::kick_off,
    crate::tool::agent::spawn::ephemeral::run,
);
