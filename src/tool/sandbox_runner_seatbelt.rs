//! macOS Seatbelt runner: confine a command with `sandbox-exec`.

use super::sandbox_runner::RunnerPlan;
use super::{SandboxPolicy, sandbox_seatbelt};
use anyhow::{Context, Result};
use std::path::Path;

/// Plan a `sandbox-exec`-confined execution of `command`.
///
/// The generated SBPL profile is written to a per-run temp file because
/// `sandbox-exec -p` truncates long inline profiles on some macOS releases.
pub(super) fn plan(
    path: &Path,
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &Path,
) -> Result<RunnerPlan> {
    let temp_dir = std::env::temp_dir();
    let profile = sandbox_seatbelt::profile(policy, work_dir, &temp_dir);
    let profile_path = sandbox_seatbelt::write_profile(&profile, &temp_dir)
        .context("Failed to stage Seatbelt profile")?;
    Ok(RunnerPlan {
        program: path.display().to_string(),
        args: exec_args(&profile_path, command, args),
        unsafe_fallbacks: sandbox_seatbelt::gaps(policy),
        network_isolated: !policy.allow_network,
        seccomp: None,
        apply_seccomp: false,
        landlock: None,
    })
}

fn exec_args(profile_path: &Path, command: &str, args: &[String]) -> Vec<String> {
    let mut out = vec![
        "-f".to_string(),
        profile_path.display().to_string(),
        command.to_string(),
    ];
    out.extend(args.iter().cloned());
    out
}

#[cfg(test)]
#[path = "sandbox_runner_seatbelt_tests.rs"]
mod tests;