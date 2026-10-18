//! Bind the shared policy-gated Git operation to the current Ralph run.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn commit_story(
        &self,
        story: &UserStory,
    ) -> anyhow::Result<()> {
        codetether_ralph_git::commit_story::<git_policy::Native>(
            &self.state.working_dir,
            &self.run_id,
            story,
        )
    }
}
