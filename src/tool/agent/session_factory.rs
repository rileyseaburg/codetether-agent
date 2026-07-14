//! Session construction helpers for spawned agents.
//!
//! Creates an initialized sub-agent [`Session`] with:
//! - a scoped **agent name** for routing messages,
//! - a **model** override distinct from the parent,
//! - a **system prompt** that inherits the parent's working directory so the
//!   sub-agent does not waste turns rediscovering the workspace.
//!
//! Keeping this separate from [`super::spawn_validation`] and
//! [`super::spawn_store`] enforces SRP: construction is one concern, policy
//! checks and persistence are others.
//!
//! # Examples
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! // `create_agent_session` is crate-private; this snippet illustrates how the
//! // module is wired from `handle_spawn`.
//! # async fn demo() -> anyhow::Result<()> {
//! # use codetether_agent::session::Session;
//! // inside crate::tool::agent::spawn::handle_spawn:
//! // let session = create_agent_session("reviewer", "Audit the PR", "glm-5.1", None, true).await?;
//! # Ok(()) }
//! # });
//! ```

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::{Context, Result};
use std::path::PathBuf;

#[path = "session_factory/parent_policy.rs"]
mod parent_policy;
#[path = "session_factory/system_message.rs"]
mod system_message;
use system_message::build as build_system_message;

/// Create a fresh [`Session`] for a spawned sub-agent.
///
/// The session is initialized with:
/// - `agent_name` set to `name` (used by the TUI / bus for message routing),
/// - `metadata.model` set to `model` (independent of the parent's model),
/// - `metadata.directory` set to the parent's workspace/worktree when provided,
/// - a single system message built by [`build_system_message`] that embeds the
///   same workspace plus explicit guidance not to spend turns on workspace
///   discovery.
///
/// # Arguments
///
/// * `name` — Sub-agent identifier, e.g. `"reviewer"`. Referenced by users as `@reviewer`.
/// * `instructions` — Free-form task description merged into the system prompt.
/// * `model` — Provider model id, e.g. `"zai/glm-5.1"`.
/// * `parent_workspace` — Workspace/worktree inherited from the parent session.
/// * `prior_context_allowed` — Whether the parent permits memory/session/history access.
///
/// # Returns
///
/// A fully initialized [`Session`] ready to be persisted via
/// [`super::spawn_store::persist_spawned_agent`].
///
/// # Errors
///
/// Returns [`anyhow::Error`] if [`Session::new`] fails (typically disk I/O
/// when initializing the session directory).
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// # async fn demo() -> anyhow::Result<()> {
/// // Crate-private: shown here for illustration. Real callers live in
/// // crate::tool::agent::spawn.
/// // let session = create_agent_session(
/// //     "tui-cache-fix",
/// //     "Apply the cache-clone fix in src/tui/app/state.rs and verify cargo check.",
/// //     "zai/glm-5.1",
/// //     Some(std::path::PathBuf::from("/workspace/project")),
/// //     true,
/// // ).await?;
/// // assert_eq!(session.metadata.model.as_deref(), Some("zai/glm-5.1"));
/// # Ok(()) }
/// # });
/// ```
pub(super) async fn create_agent_session(
    name: &str,
    instructions: &str,
    model: &str,
    parent_workspace: Option<PathBuf>,
    prior_context_allowed: bool,
) -> Result<Session> {
    let mut session = Session::new().await.context("Failed to create session")?;
    let workspace = parent_workspace.or_else(|| session.metadata.directory.clone());
    session.set_agent_name(name.to_string());
    session.set_title(format!("Sub-agent: {name}"));
    session.metadata.model = Some(model.to_string());
    session.metadata.directory = workspace.clone();
    session.metadata.inherited_prior_context_allowed = Some(prior_context_allowed);
    // Spawned sub-agents are autonomous and cannot interactively confirm edits,
    // so edit previews must be auto-applied or they spin re-issuing the same
    // pending edit forever (see issue #294).
    session.metadata.auto_apply_edits = true;
    session.bus = crate::bus::global();
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: build_system_message(name, instructions, workspace),
        }],
    });
    Ok(session)
}

/// Resolve the current parent policy for propagation to a child session.
pub(super) async fn parent_prior_context_allowed(
    inherited: Option<bool>,
    parent_id: Option<&str>,
) -> bool {
    parent_policy::resolve(inherited, parent_id).await
}

#[cfg(test)]
#[path = "session_factory/policy_tests.rs"]
mod policy_tests;
#[cfg(test)]
mod tests;
