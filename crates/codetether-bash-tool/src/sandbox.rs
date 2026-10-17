//! Run a command through the shared OS sandbox.
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_sandbox_runtime::execute_sandboxed;
pub(super) async fn execute(call: Call<'_>) -> Result<ToolResult> {
    let sandbox_root = codetether_bash_policy::mode::root(call.effective_cwd.clone());
    let policy =
        codetether_bash_policy::policy::policy(sandbox_root.clone(), call.timeout_secs).await;
    let work_dir = sandbox_root.as_path();
    let shell = codetether_command_shell::resolve();
    let mut sandbox_args: Vec<String> = shell.prefix_args.clone();
    sandbox_args.push(call.wrapped_command.clone());
    let sandbox_result =
        execute_sandboxed(&shell.program, &sandbox_args, &policy, Some(work_dir)).await;

    super::sandbox_audit::record(call.command, &sandbox_result).await;
    match sandbox_result {
        Ok(r) => super::sandbox_finished::finish(&call, &policy, work_dir, r),
        Err(e) => super::sandbox_error::finish(&call, &policy, work_dir, e),
    }
}
