use super::super::sandbox_runner::RunnerPlan;
use super::super::{SandboxPolicy, sandbox_landlock, sandbox_seccomp};

/// Direct execution confined by Landlock (a kernel LSM applied via
/// `pre_exec`). Landlock needs no user namespace, so it enforces the
/// filesystem policy on hosts where bwrap's smoke probe fails. Returns
/// `None` when the kernel cannot enforce Landlock, so the caller still
/// fails closed instead of running an unconfined process.
pub(super) fn confined_plan(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &std::path::Path,
    _reason: &str,
) -> Option<RunnerPlan> {
    let rules = sandbox_landlock::prepare(policy, work_dir).rules?;
    let seccomp = sandbox_seccomp::prepare(policy.allow_network).ok().flatten()?;
    Some(RunnerPlan {
        program: command.to_string(),
        args: args.to_vec(),
        unsafe_fallbacks: Vec::new(),
        landlock: Some(rules),
        network_isolated: !policy.allow_network,
        seccomp: Some(seccomp),
        apply_seccomp: true,
    })
}