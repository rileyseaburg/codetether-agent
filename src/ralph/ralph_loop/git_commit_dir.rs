//! commit_in_dir application policy and provenance integration.
use super::*;
impl RalphLoop {
    /// Commit changes in a specific directory
    pub(super) fn commit_in_dir(&self, dir: &PathBuf, story: &UserStory) -> anyhow::Result<()> {
        git_policy::guard(dir, "git add -A")?;

        // Commit with story reference
        let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
        let provenance = self.commit_provenance();
        git_policy::guard(dir, &format!("git commit -m {msg}"))?;
        let _ = git_commit_with_provenance_blocking(dir, &msg, Some(&provenance));

        Ok(())
    }
}
