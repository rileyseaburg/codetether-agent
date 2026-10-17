//! commit_story application policy and provenance integration.
use super::*;
impl RalphLoop {
    /// Commit changes for a story
    pub(in crate::ralph::ralph_loop) fn commit_story(
        &self,
        story: &UserStory,
    ) -> anyhow::Result<()> {
        info!(target: "codetether_agent::ralph::ralph_loop::git_commit", "Committing changes for story: {}", story.id);

        git_policy::guard(&self.state.working_dir, "git add -A")?;

        // Commit with story reference
        let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
        let provenance = self.commit_provenance();
        git_policy::guard(&self.state.working_dir, &format!("git commit -m {msg}"))?;
        match git_commit_with_provenance_blocking(&self.state.working_dir, &msg, Some(&provenance))
        {
            Ok(output) if output.status.success() => {
                info!(target: "codetether_agent::ralph::ralph_loop::git_commit", "Committed: {}", msg);
            }
            Ok(output) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::git_commit",
                    "Git commit had no changes or failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::git_commit", "Could not run git commit: {}", e);
            }
        }

        Ok(())
    }
}
