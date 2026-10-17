//! commit_story application policy and provenance integration.
use super::*;
impl RalphLoop {
    /// Commit changes for a story
    pub(super) fn commit_story(&self, story: &UserStory) -> anyhow::Result<()> {
        info!("Committing changes for story: {}", story.id);

        git_policy::guard(&self.state.working_dir, "git add -A")?;

        // Commit with story reference
        let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
        let provenance = self.commit_provenance();
        git_policy::guard(&self.state.working_dir, &format!("git commit -m {msg}"))?;
        match git_commit_with_provenance_blocking(&self.state.working_dir, &msg, Some(&provenance))
        {
            Ok(output) if output.status.success() => {
                info!("Committed: {}", msg);
            }
            Ok(output) => {
                warn!(
                    "Git commit had no changes or failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                warn!("Could not run git commit: {}", e);
            }
        }

        Ok(())
    }
}
