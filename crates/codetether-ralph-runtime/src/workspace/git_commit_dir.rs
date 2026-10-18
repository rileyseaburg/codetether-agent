//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) fn commit_in_dir(&self, dir: &PathBuf, story: &UserStory) -> anyhow::Result<()> {
        codetether_ralph_git::commit_in_dir::<H::Git>(dir, &self.run_id, story)
    }
}
