//! Persist a still-running command under its authoritative owner.

use super::super::ExecCommandTool;
use crate::tool::ToolResult;
use crate::tool::command_session::{self, Poll, Running};
use anyhow::Result;
use serde_json::Value;

pub(super) async fn result(
    tool: &ExecCommandTool,
    command: Running,
    poll: Poll,
    args: &Value,
) -> Result<ToolResult> {
    if !poll.running {
        return Ok(command_session::tool_result(
            poll,
            &command.metadata,
            None,
        ));
    }
    let Some(owner) = crate::tool::command_owner::from_args(args) else {
        return Ok(ToolResult::error(
            "persistent command sessions require a session or lease owner",
        ));
    };
    let id = tool.sessions.insert(command, owner.clone()).await?;
    let running = tool.sessions.get(id, &owner).await.expect("inserted command");
    let command = running.lock().await;
    Ok(command_session::tool_result(poll, &command.metadata, Some(id)))
}