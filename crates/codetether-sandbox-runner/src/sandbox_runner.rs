//! runner for sandbox execution planning.
use super::sandbox_runner_select::{Runner, selected_runner};
use super::{SandboxPolicy, sandbox_landlock, sandbox_seccomp};
use anyhow::Result;
use std::path::Path;

/// Owned executable, arguments and confinement resources for one launch.
///
/// # Examples
/// ```rust,no_run
/// let policy = codetether_sandbox_policy::SandboxPolicy::default();
/// let _plan = codetether_sandbox_runner::plan("example", &[], &policy, std::path::Path::new("/workspace"))?;
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug)]
pub struct RunnerPlan {
    pub program: String,
    pub args: Vec<String>,
    pub unsafe_fallbacks: Vec<String>,
    pub network_isolated: bool,
    pub _seccomp: Option<sandbox_seccomp::Program>,
    pub landlock: Option<sandbox_landlock::Rules>,
}

/// Build a launch plan while retaining confinement resources.
///
/// # Arguments
/// * `command` - Child executable.
/// * `args` - Child arguments.
/// * `policy` - Filesystem and network permissions.
/// * `work_dir` - Child working directory.
/// # Returns
/// The launch plan and owned kernel/profile resources.
/// # Errors
/// Returns preparation errors or refuses unavailable confinement without an override.
pub fn plan(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    work_dir: &Path,
) -> Result<RunnerPlan> {
    if work_dir.is_absolute() {
        selected_runner().plan(command, args, policy, work_dir)
    } else {
        Runner::Direct("relative_working_dir").plan(command, args, policy, work_dir)
    }
}

impl Runner {
    pub fn plan(
        &self,
        command: &str,
        args: &[String],
        policy: &SandboxPolicy,
        work_dir: &Path,
    ) -> Result<RunnerPlan> {
        match self {
            Self::Bubblewrap(path) => {
                super::sandbox_runner_bwrap::plan(path, command, args, policy, work_dir)
            }
            Self::Seatbelt(path) => {
                super::sandbox_runner_seatbelt::plan(path, command, args, policy, work_dir)
            }
            Self::Direct(reason) => {
                super::sandbox_runner_direct::plan(command, args, policy, work_dir, reason)
            }
        }
    }
}
