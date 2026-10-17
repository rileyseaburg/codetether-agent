//! Initialization defaults for a newly spawned managed child.
use crate::ChildSession;
use anyhow::{Context, Result};
use codetether_provider_core::{ContentPart, Message, Role};
use std::path::PathBuf;
/// Construct and initialize a child with the parent's workspace and policy.
/// # Arguments
/// * `name` — Child display name.
/// * `instructions` — Task text embedded in the system prompt.
/// * `model` — Child model override.
/// * `parent_workspace` — Explicit workspace or the new session's default.
/// * `prior_context_allowed` — Inherited access ceiling.
/// # Returns
/// An initialized native session, not additionally persisted by this function.
/// # Errors
/// Propagates session creation errors with the original context.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_session::{create, ChildSession};
/// async fn example<S: ChildSession>() -> anyhow::Result<S> {
///     create::<S>("reviewer", "review", "provider/model", None, false).await
/// }
/// ```
pub async fn create<S: ChildSession>(
    name: &str,
    instructions: &str,
    model: &str,
    parent_workspace: Option<PathBuf>,
    prior_context_allowed: bool,
) -> Result<S> {
    let mut session = S::create().await.context("Failed to create session")?;
    let workspace = parent_workspace.or_else(|| session.settings().directory.clone());
    session.set_agent_name(name.to_string());
    session.set_title(format!("Sub-agent: {name}"));
    *session.settings().model = Some(model.to_string());
    *session.settings().directory = workspace.clone();
    *session.settings().inherited_prior_context_allowed = Some(prior_context_allowed);
    // Spawned sub-agents are autonomous and cannot interactively confirm edits,
    // so edit previews must be auto-applied or they spin re-issuing the same
    // pending edit forever (see issue #294).
    *session.settings().auto_apply_edits = true;
    session.attach_bus();
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: codetether_agent_messages::system_message(name, instructions, workspace),
        }],
    });
    Ok(session)
}
