//! Synchronous sandboxed process creation for blocking plugin runtimes.

use super::{SandboxPolicy, sandbox_network_isolation, sandbox_runner};
use anyhow::{Context, Result, bail};
use std::path::Path;
use std::process::Child;
#[path = "sandbox_spawn_std_command.rs"]
mod command;
#[path = "sandbox_spawn_std_env.rs"]
mod environment;
pub(crate) use environment::{spawn_with_env, spawn_with_env_input};

pub(crate) fn preflight(policy: &SandboxPolicy, cwd: &Path) -> Result<()> {
    validate_cwd(policy, cwd)?;
    let args = Vec::new();
    let plan = sandbox_runner::plan("true", &args, policy, cwd)?;
    sandbox_network_isolation::validate(policy, plan.network_isolated)?;
    Ok(())
}

pub(crate) fn spawn(
    program: &str,
    args: &[String],
    policy: &SandboxPolicy,
    cwd: &Path,
    has_stdin: bool,
) -> Result<Child> {
    validate_cwd(policy, cwd)?;
    crate::tool::sandbox_network::validate(policy, program, args)?;
    crate::tool::sandbox_paths::validate_command_args(args)?;
    let plan = sandbox_runner::plan(program, args, policy, cwd)?;
    sandbox_network_isolation::validate(policy, plan.network_isolated)?;
    command::spawn(
        &plan.program,
        &plan.args,
        cwd,
        has_stdin,
        &[],
        plan.landlock,
        plan.seccomp.as_ref(),
        plan.apply_seccomp,
    )
}

fn validate_cwd(policy: &SandboxPolicy, cwd: &Path) -> Result<()> {
    let cwd = cwd.canonicalize().context("failed to resolve plugin workspace")?;
    if !policy.allowed_paths.is_empty()
        && !policy.allowed_paths.iter().filter_map(|path| path.canonicalize().ok()).any(|root| cwd.starts_with(root))
    {
        bail!("plugin process working directory is outside the sandbox workspace");
    }
    Ok(())
}