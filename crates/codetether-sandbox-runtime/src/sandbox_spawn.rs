//! Sandboxed child creation for persistent command sessions.

use super::{SandboxPolicy, sandbox_command, sandbox_env, sandbox_plan_state, sandbox_runner};
use anyhow::{Context, Result, anyhow};
use std::path::Path;

/// A live sandboxed child and the isolation fallbacks applied to it.
///
/// Returned by [crate::spawn]; keep the child and terminal alive for I/O.
///
/// # Examples
/// ```rust,no_run
/// # async fn example(policy: &codetether_sandbox_policy::SandboxPolicy) -> anyhow::Result<()> {
/// let _child = codetether_sandbox_runtime::spawn(
///     "example", &[], policy, std::path::Path::new("/workspace"), false, &[],
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub struct Spawned {
    pub child: tokio::process::Child,
    pub terminal: Option<codetether_command_pty::Attached>,
    pub unsafe_fallbacks: Vec<String>,
}

/// Validate, sandbox, and spawn a command without waiting for completion.
///
/// # Arguments
/// The executable, arguments, policy, working directory, interactive-input flag,
/// and environment overrides define the child invocation.
/// # Returns
/// A live child, optional terminal and applied confinement-gap labels.
/// # Errors
/// Returns validation, planning, terminal or process-spawn errors.
pub async fn spawn(
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
    let mut env = sandbox_env::restricted(policy);
    let network = codetether_sandbox_policy::validate_network(policy, command, args)?;
    if !policy.allow_network {
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".into(), "1".into());
    }
    env.extend(environment.iter().cloned());
    codetether_sandbox_policy::validate_working_dir(policy, working_dir).await?;
    codetether_sandbox_policy::validate_command_args(args)?;
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
        .then(|| codetether_command_pty::attach(&mut cmd))
        .transpose()?;
    let mut unsafe_fallbacks = state.unsafe_fallbacks;
    unsafe_fallbacks.extend(limits);
    // sandbox_command::build already installed the process-group hook.
    let child = cmd.spawn().context("Failed to spawn sandboxed process")?;
    Ok(Spawned {
        child,
        terminal,
        unsafe_fallbacks,
    })
}
