//! commit_story application policy and provenance integration.
use super::*;
/// Commit story changes with run provenance and the original outcome logging.
/// # Arguments
/// The repository path, run identity and completed story.
/// # Returns
/// Returns after the commit attempt; Git failures retain their original logged handling.
/// # Errors
/// Returns policy rejection or evaluator errors.
/// # Examples
/// ```rust,no_run
/// # fn example<H: codetether_ralph_git::Host>(path: &std::path::Path, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// codetether_ralph_git::commit_story::<H>(path, "run-1", story)?;
/// # Ok(()) }
/// ```
pub fn commit_story<H: Host>(cwd: &Path, run_id: &str, story: &UserStory) -> anyhow::Result<()> {
    info!(target: "codetether_agent::ralph::ralph_loop::git_commit", "Committing changes for story: {}", story.id);

    crate::policy::guard::<H>(cwd, "git add -A")?;

    // Commit with story reference
    let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
    let provenance = commit_provenance(run_id);
    crate::policy::guard::<H>(cwd, &format!("git commit -m {msg}"))?;
    match git_commit_with_provenance_blocking(cwd, &msg, Some(&provenance)) {
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
