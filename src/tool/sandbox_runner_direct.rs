use super::sandbox_runner::RunnerPlan;
use anyhow::{Result, anyhow};

pub(super) const ENV: &str = "CODETETHER_ALLOW_UNSAFE_SANDBOX_FALLBACK";

pub(super) fn plan(command: &str, args: &[String], reason: &str) -> Result<RunnerPlan> {
    plan_with_override(command, args, reason, allowed())
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

pub(super) fn enabled(value: Option<&str>) -> bool {
    value.is_some_and(truthy)
}

fn allowed() -> bool {
    enabled(std::env::var(ENV).ok().as_deref())
}

fn denial(reason: &str) -> String {
    format!("OS sandbox unavailable ({reason}); refusing direct execution; set {ENV}=1")
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
#[path = "sandbox_runner_direct_tests.rs"]
mod tests;
