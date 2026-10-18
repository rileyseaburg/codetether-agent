//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn git_checkout(&self, branch: &str) -> anyhow::Result<()> {
        codetether_ralph_git::checkout_or_create::<git_policy::Native>(
            &self.state.working_dir,
            branch,
        )
    }
}
