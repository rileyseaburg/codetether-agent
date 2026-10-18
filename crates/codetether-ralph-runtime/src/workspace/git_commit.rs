//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl<H: Host> RalphLoop<H> {
    pub(crate) fn commit_story(&self, story: &UserStory) -> anyhow::Result<()> {
        codetether_ralph_git::commit_story::<H::Git>(&self.state.working_dir, &self.run_id, story)
    }
}
