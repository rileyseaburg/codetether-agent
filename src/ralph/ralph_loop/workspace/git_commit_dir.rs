//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn commit_in_dir(
        &self,
        dir: &PathBuf,
        story: &UserStory,
    ) -> anyhow::Result<()> {
        codetether_ralph_git::commit_in_dir::<git_policy::Native>(dir, &self.run_id, story)
    }
}
