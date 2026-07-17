//! One exec_command invocation from validation through initial yield.

use anyhow::Result;
use serde_json::Value;

use super::{ExecCommandTool, input::Input};
use crate::tool::{ToolResult, command_session};

pub(super) async fn run(tool: &ExecCommandTool, args: Value) -> Result<ToolResult> {
    let input: Input = match serde_json::from_value(args.clone()) {
        Ok(input) => input,
        Err(error) => {
            return Ok(ToolResult::error(format!(
                "invalid exec_command input: {error}"
            )));
        }
    };
    if input.cmd.trim().is_empty() {
        return Ok(ToolResult::error("cmd must not be empty"));
    }
    if let Some(blocked) = crate::tool::shell_command_guard::result("exec_command", &input.cmd) {
        return Ok(blocked);
    }
    let cwd = super::shell::cwd(tool.default_cwd.as_deref(), input.workdir.as_deref())?;
    let (program, command_args) = super::shell::invocation(&input);
    let policy = super::policy::resolve(&input.cmd, &args, &cwd).await;
    let environment = super::environment::resolve(&input.cmd, &cwd, &args).await;
    let mut command = command_session::command(
        &program,
        &command_args,
        &cwd,
        input.tty,
        &environment.variables,
        policy.as_ref(),
    )
    .await?;
    command.metadata.redactions = environment.redactions;
    let poll = command.poll(input.yield_ms(), input.max_bytes()).await?;
    if poll.running {
        let id = tool.sessions.insert(command).await?;
        let running = tool.sessions.get(id).await.expect("inserted command");
        let command = running.lock().await;
        return Ok(command_session::tool_result(
            poll,
            &command.metadata,
            Some(id),
        ));
    }
    Ok(command_session::tool_result(poll, &command.metadata, None))
}
