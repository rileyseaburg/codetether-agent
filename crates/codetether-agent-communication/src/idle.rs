//! Durable queue-only delivery when a child has no active turn.

use crate::{Host, Route};
use anyhow::Result;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_tool_core::ToolResult;

/// Steer an active child or persist a message for its next turn.
/// # Arguments
/// * `target` — Child name or durable ID.
/// * `owner` — Owning session scope, or the existing unscoped lookup mode.
/// * `message` — User text to deliver or persist.
/// # Returns
/// An empty successful tool result after delivery, or an unsuccessful result
/// when the child is missing. Idle delivery does not start a new turn.
/// # Errors
/// Propagates the native session save error without updating the live snapshot.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_communication::{Host, queue_only};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let result = queue_only::<H>("reviewer", Some("parent"), "Context".into()).await?;
/// # Ok(()) }
/// ```
pub async fn queue_only<H: Host>(
    target: &str,
    owner: Option<&str>,
    message: String,
) -> Result<ToolResult> {
    match crate::steer::<H>(target, owner, &message).await {
        Route::Steered => return Ok(ToolResult::success(String::new())),
        Route::NotFound => return Ok(ToolResult::error(format!("Agent {target} not found"))),
        Route::Idle => {}
    }
    let Some(agent_id) = H::resolve(target, owner) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let Some(mut session) = H::session(&agent_id) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    H::add_human_message(
        &mut session,
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: message }],
        },
    );
    H::save(&session).await?;
    H::update_session(&agent_id, session);
    Ok(ToolResult::success(String::new()))
}
