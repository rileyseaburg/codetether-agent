//! One write or poll against an existing command session.

use anyhow::Result;

use super::{WriteStdinTool, input::Input};
use crate::tool::{ToolResult, command_session};

pub(super) async fn run(tool: &WriteStdinTool, args: serde_json::Value) -> Result<ToolResult> {
    let input: Input = match serde_json::from_value(args) {
        Ok(input) => input,
        Err(error) => {
            return Ok(ToolResult::error(format!(
                "invalid write_stdin input: {error}"
            )));
        }
    };
    let Some(command) = tool.sessions.get(input.session_id).await else {
        return Ok(ToolResult::error(format!(
            "unknown or completed command session {}",
            input.session_id
        )));
    };
    let mut command = command.lock().await;
    if !input.chars.is_empty()
        && let Err(error) = command.write(&input.chars).await
    {
        return Ok(ToolResult::error(error.to_string()));
    }
    let poll = command.poll(input.yield_ms(), input.max_bytes()).await?;
    let running = poll.running;
    let result = command_session::tool_result(poll, &command.metadata, Some(input.session_id));
    drop(command);
    if !running {
        tool.sessions.remove(input.session_id).await;
    }
    Ok(result)
}
