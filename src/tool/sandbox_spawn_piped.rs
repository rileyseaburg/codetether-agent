//! Sandboxed child creation with JSON-RPC-compatible stdio pipes.

use super::{SandboxPolicy, sandbox_env, sandbox_plan_command, sandbox_plan_state, sandbox_runner};
use anyhow::{Context, Result, anyhow};
use std::path::Path;
use std::process::Stdio;

pub(crate) async fn spawn(
    program: &str,
    args: &[String],
    policy: &SandboxPolicy,
    cwd: &Path,
) -> Result<tokio::process::Child> {
    if !policy.allow_exec {
        return Err(anyhow!("Sandbox policy denies process execution"));
    }
    let mut env = sandbox_env::restricted();
    env.extend(policy.environment.clone());
    let network = crate::tool::sandbox_network::validate(policy, program, args)?;
    if !policy.allow_network {
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".into(), "1".into());
    }
    crate::tool::sandbox_paths::validate_working_dir(policy, cwd).await?;
    crate::tool::sandbox_paths::validate_command_args(args)?;
    let plan = sandbox_runner::plan(program, args, policy, cwd)?;
    super::sandbox_network_isolation::validate(policy, plan.network_isolated)?;
    let mut state = sandbox_plan_state::from_plan(plan, network);
    let (mut command, limits) =
        sandbox_plan_command::build(&mut state, cwd, &env, policy.max_memory_bytes);
    if !limits.is_empty() {
        tracing::warn!(fallbacks = ?limits, "Piped sandbox has advisory resource limits");
    }
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("Failed to spawn sandboxed piped process")
}