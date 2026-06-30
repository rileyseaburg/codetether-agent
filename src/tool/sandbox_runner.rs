use super::sandbox_runner_select::{Runner, selected_runner};
use super::{SandboxPolicy, sandbox_landlock, sandbox_seccomp};
use anyhow::Result;
use std::path::Path;

#[derive(Debug)]
pub(super) struct RunnerPlan {
    pub program: String,
    pub args: Vec<String>,
    pub unsafe_fallbacks: Vec<String>,
    pub network_isolated: bool,
    pub _seccomp: Option<sandbox_seccomp::Program>,
    pub landlock: Option<sandbox_landlock::Rules>,
}

pub(super) fn plan(
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
    pub(super) fn plan(
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
            Self::Direct(reason) => {
                super::sandbox_runner_direct::plan(command, args, policy, work_dir, reason)
            }
        }
    }
}

#[cfg(test)]
#[path = "sandbox_runner_tests.rs"]
mod tests;
