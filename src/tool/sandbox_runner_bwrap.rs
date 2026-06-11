use super::sandbox_runner::RunnerPlan;
use super::{SandboxPolicy, sandbox_bwrap_args, sandbox_landlock, sandbox_seccomp};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(super) fn plan(
    path: &PathBuf,
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &Path,
) -> Result<RunnerPlan> {
    let seccomp = sandbox_seccomp::prepare(policy.allow_network)?;
    let seccomp_fd = seccomp.as_ref().map(sandbox_seccomp::Program::fd);
    let landlock = sandbox_landlock::prepare(policy, work_dir);
    let mut unsafe_fallbacks =
        super::sandbox_bwrap_probe::kernel_fallbacks(seccomp.is_some(), landlock.rules.is_some());
    unsafe_fallbacks.extend(landlock.fallback);
    Ok(RunnerPlan {
        program: path.display().to_string(),
        args: sandbox_bwrap_args::build(command, args, policy, work_dir, seccomp_fd),
        unsafe_fallbacks,
        network_isolated: !policy.allow_network,
        _seccomp: seccomp,
        landlock: landlock.rules,
    })
}
