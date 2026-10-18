//! commit_in_dir application policy and provenance integration.
use super::*;
/// Commit isolated story changes with run provenance.
/// # Arguments
/// The isolated workspace, run identity and completed story.
/// # Returns
/// Returns after the commit attempt, preserving ignored Git failures.
/// # Errors
/// Returns policy rejection or evaluator errors.
/// # Examples
/// ```rust,no_run
/// # fn example<H: codetether_ralph_git::Host>(path: &std::path::Path, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// codetether_ralph_git::commit_in_dir::<H>(&path.to_path_buf(), "run-1", story)?;
/// # Ok(()) }
/// ```
pub fn commit_in_dir<H: Host>(
    dir: &PathBuf,
    run_id: &str,
    story: &UserStory,
) -> anyhow::Result<()> {
    crate::policy::guard::<H>(dir, "git add -A")?;

    // Commit with story reference
    let msg = format!("feat({}): {}", story.id.to_lowercase(), story.title);
    let provenance = commit_provenance(run_id);
    crate::policy::guard::<H>(dir, &format!("git commit -m {msg}"))?;
    let _ = git_commit_with_provenance_blocking(dir, &msg, Some(&provenance));

    Ok(())
}
