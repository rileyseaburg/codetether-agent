//! Environment-aware synchronous sandbox spawning.

use crate::tool::sandbox::SandboxPolicy;
use anyhow::Result;
use std::path::Path;
use std::process::Child;

pub(crate) fn spawn_with_env(
    program: &str, args: &[String], policy: &SandboxPolicy, cwd: &Path,
    environment: &[(String, String)],
) -> Result<Child> {
    super::validate_cwd(policy, cwd)?;
    crate::tool::sandbox_network::validate(policy, program, args)?;
    crate::tool::sandbox_paths::validate_command_args(args)?;
    let plan = super::sandbox_runner::plan(program, args, policy, cwd)?;
    super::sandbox_network_isolation::validate(policy, plan.network_isolated)?;
    super::command::spawn(
        &plan.program,
        &plan.args,
        cwd,
        false,
        environment,
        plan.landlock,
        plan.seccomp.as_ref(),
        plan.apply_seccomp,
    )
}

pub(crate) fn spawn_with_env_input(
    program: &str,
    args: &[String],
    policy: &SandboxPolicy,
    cwd: &Path,
    environment: &[(String, String)],
) -> Result<Child> {
    super::validate_cwd(policy, cwd)?;
    crate::tool::sandbox_network::validate(policy, program, args)?;
    crate::tool::sandbox_paths::validate_command_args(args)?;
    let plan = super::sandbox_runner::plan(program, args, policy, cwd)?;
    super::sandbox_network_isolation::validate(policy, plan.network_isolated)?;
    super::command::spawn(
        &plan.program,
        &plan.args,
        cwd,
        true,
        environment,
        plan.landlock,
        plan.seccomp.as_ref(),
        plan.apply_seccomp,
    )
}