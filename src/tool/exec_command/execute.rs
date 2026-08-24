//! One exec_command invocation from validation through initial yield.

use anyhow::Result;
use serde_json::Value;

use super::{ExecCommandTool, input::Input};
use crate::tool::{ToolResult, command_session};

#[path = "execute/validate.rs"]
mod validate;
#[path = "execute/approval.rs"]
mod approval;
#[path = "execute/persist.rs"]
mod persist;
#[path = "execute/workspace.rs"]
mod workspace;

pub(super) async fn run(tool: &ExecCommandTool, args: Value) -> Result<ToolResult> {
    let mut args = validate::bind_default_workdir(args, tool.default_cwd.as_deref());
    let allowed = crate::tool::network_access::allowed_for(&args);
    crate::tool::network_access::bind(&mut args, allowed);
    if let Some(blocked) = validate::runtime_policy(&args).await {
        return Ok(blocked);
    }
    let input: Input = match validate::input(&args) {
        Ok(input) => input,
        Err(result) => return Ok(result),
    };
    let root = workspace::resolve(&args, tool.default_cwd.as_deref());
    let cwd = super::shell::cwd(root.as_deref(), input.workdir.as_deref())?;
    let (program, command_args) = super::shell::invocation(&input);
    let policy = super::policy::resolve(&input.cmd, &args, &cwd).await;
    if let Err(result) = approval::validate(&args, (&program, &command_args), policy.as_ref(), &cwd).await {
        return Ok(result);
    }
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
    persist::result(tool, command, poll, &args).await
}