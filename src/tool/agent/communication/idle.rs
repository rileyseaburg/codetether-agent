//! Durable queue-only delivery when a child has no active turn.

use super::Route;
use crate::provider::{ContentPart, Message, Role};
use crate::tool::ToolResult;
use anyhow::Result;

pub(crate) async fn queue_only(
    target: &str,
    owner: Option<&str>,
    message: String,
) -> Result<ToolResult> {
    match super::steer(target, owner, &message).await {
        Route::Steered => return Ok(ToolResult::success(String::new())),
        Route::NotFound => return Ok(ToolResult::error(format!("Agent {target} not found"))),
        Route::Idle => {}
    }
    let Some((agent_id, _)) = super::super::store::scope::resolve_for_parent(target, owner) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let Some(entry) = super::super::store::get(&agent_id) else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let mut session = entry.session;
    session.add_human_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: message }],
    });
    session.save().await?;
    super::super::store::update_session(&agent_id, session);
    Ok(ToolResult::success(String::new()))
}
