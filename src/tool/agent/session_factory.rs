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
//! // let session = create_agent_session("reviewer", "Audit the PR", "glm-5.1").await?;
//! # Ok(()) }
//! # });
//! ```

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::{Context, Result};
use std::path::PathBuf;

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
) -> Result<Session> {
    let mut session = Session::new().await.context("Failed to create session")?;
    let workspace = parent_workspace.or_else(|| session.metadata.directory.clone());
    session.set_agent_name(name.to_string());
    session.metadata.model = Some(model.to_string());
    session.metadata.directory = workspace.clone();
    session.bus = crate::bus::global();
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: build_system_message(name, instructions, workspace),
        }],
    });
    Ok(session)
}

/// Build the system prompt injected into every spawned sub-agent.
///
/// The prompt embeds the same workspace stored on the spawned child session so
/// the sub-agent can resolve relative paths immediately, and includes explicit
/// directives to avoid workspace-discovery tool calls (`pwd`, `ls`, `glob`)
/// that were observed burning 4–6 turns before any real edit.
///
/// If no workspace was passed from the parent, the directory falls back to
/// [`std::env::current_dir`] and then the literal string `"<unknown>"`. This
/// never panics.
///
/// # Arguments
///
/// * `name` — Sub-agent identifier injected as `@{name}`.
/// * `instructions` — Task description appended to the role preamble.
/// * `workspace` — Resolved parent workspace/worktree.
///
/// # Returns
///
/// A multi-line system prompt string.
fn build_system_message(name: &str, instructions: &str, workspace: Option<PathBuf>) -> String {
    let cwd = workspace
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<unknown>".to_string());
    format!(
        "You are @{name}, a specialized sub-agent. {instructions}\n\n\
         Workspace cwd: {cwd}\n\
         All file paths you read/write should be relative to this cwd unless absolute.\n\
         Do NOT waste turns discovering the workspace (no pwd/ls/glob to locate files).\n\
         Act directly: read only the files you need, make edits, verify, report pass/fail briefly.\n\
         Budget: aim for <10 tool calls for small edits; narrate minimally."
    )
}

#[cfg(test)]
mod tests;
