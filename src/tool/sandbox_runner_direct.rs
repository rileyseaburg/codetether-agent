use super::SandboxPolicy;
use super::sandbox_runner::RunnerPlan;
use anyhow::{Result, anyhow};
use std::path::Path;

#[path = "sandbox_runner_direct_confined.rs"]
mod confined;
#[path = "sandbox_runner_direct_env.rs"]
mod env;

pub(in crate::tool::sandbox) use env::enabled;

pub(super) const ENV: &str = "CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK";

pub(super) fn plan(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &Path,
    reason: &str,
) -> Result<RunnerPlan> {
    if let Some(plan) = confined::confined_plan(command, args, policy, work_dir, reason) {
        return Ok(plan);
    }
    plan_with_override(command, args, reason, env::allowed(ENV))
}

pub(super) fn plan_with_override(
    command: &str,
    args: &[String],
    reason: &str,
    allow_unsafe: bool,
) -> Result<RunnerPlan> {
    if !allow_unsafe {
        return Err(anyhow!("{}", denial(reason)));
    }
    let mut unsafe_fallbacks = super::sandbox_bwrap_probe::direct_fallbacks(reason);
    unsafe_fallbacks.push(format!("unsafe_sandbox_fallback_allowed:{ENV}"));
    Ok(RunnerPlan {
        program: command.to_string(),
        args: args.to_vec(),
        unsafe_fallbacks,
        network_isolated: false,
        _seccomp: None,
        landlock: None,
    })
}

fn denial(reason: &str) -> String {
    format!("OS sandbox unavailable ({reason}); refusing direct execution; set {ENV}=1")
}

#[cfg(test)]
#[path = "sandbox_runner_direct_tests.rs"]
mod tests;
