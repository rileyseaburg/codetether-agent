//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) fn git_checkout(&self, branch: &str) -> anyhow::Result<()> {
        codetether_ralph_git::checkout_or_create::<H::Git>(&self.state.working_dir, branch)
    }
}
