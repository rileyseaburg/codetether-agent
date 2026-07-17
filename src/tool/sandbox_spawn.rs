//! Sandboxed child creation for persistent command sessions.

use super::{SandboxPolicy, sandbox_command, sandbox_env, sandbox_plan_state, sandbox_runner};
use anyhow::{Context, Result, anyhow};
use std::path::Path;

/// A live sandboxed child and the isolation fallbacks applied to it.
pub(crate) struct Spawned {
    pub child: tokio::process::Child,
    pub terminal: Option<crate::tool::command_pty::Attached>,
    pub unsafe_fallbacks: Vec<String>,
}

/// Validate, sandbox, and spawn a command without waiting for completion.
pub(crate) async fn spawn(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    working_dir: &Path,
    keep_stdin: bool,
    environment: &[(String, String)],
) -> Result<Spawned> {
    if !policy.allow_exec {
        return Err(anyhow!("Sandbox policy denies process execution"));
    }
    let mut env = sandbox_env::restricted();
    let network = super::super::sandbox_network::validate(policy, command, args)?;
    if !policy.allow_network {
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".into(), "1".into());
    }
    env.extend(environment.iter().cloned());
    super::super::sandbox_paths::validate_working_dir(policy, working_dir).await?;
    super::super::sandbox_paths::validate_command_args(args)?;
    let plan = sandbox_runner::plan(command, args, policy, working_dir)?;
    let state = sandbox_plan_state::from_plan(plan, network);
    let (mut cmd, limits) = sandbox_command::build(
        &state.program,
        &state.args,
        working_dir,
        &env,
        state.landlock,
        policy.max_memory_bytes,
    );
    let terminal = keep_stdin
        .then(|| crate::tool::command_pty::attach(&mut cmd))
        .transpose()?;
    let mut unsafe_fallbacks = state.unsafe_fallbacks;
    unsafe_fallbacks.extend(limits);
    let child = cmd.spawn().context("Failed to spawn sandboxed process")?;
    Ok(Spawned {
        child,
        terminal,
        unsafe_fallbacks,
    })
}
