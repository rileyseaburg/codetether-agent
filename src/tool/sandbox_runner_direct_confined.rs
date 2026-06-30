use super::super::sandbox_runner::RunnerPlan;
use super::super::{SandboxPolicy, sandbox_landlock};

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
    reason: &str,
) -> Option<RunnerPlan> {
    let rules = sandbox_landlock::prepare(policy, work_dir).rules?;
    let base = super::plan_with_override(command, args, reason, true).ok()?;
    Some(RunnerPlan {
        landlock: Some(rules),
        network_isolated: false,
        ..base
    })
}
