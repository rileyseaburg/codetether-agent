//! git_checkout application policy and provenance integration.
use super::*;
/// Checkout a branch, retrying with branch creation after an unsuccessful exit.
/// # Arguments
/// The repository path and requested branch.
/// # Returns
/// Returns after checkout or the original branch-creation fallback.
/// # Errors
/// Returns policy rejection or process-launch errors.
/// # Examples
/// ```rust,no_run
/// # fn example<H: codetether_ralph_git::Host>(path: &std::path::Path, story: &codetether_ralph_core::UserStory) -> anyhow::Result<()> {
/// codetether_ralph_git::checkout_or_create::<H>(path, "feature")?;
/// # Ok(()) }
/// ```
pub fn git_checkout<H: Host>(cwd: &Path, branch: &str) -> anyhow::Result<()> {
    // Try to checkout, create if doesn't exist
    let output = crate::checkout::<H>(cwd, branch)?;

    if !output.status.success() {
        crate::policy::guard::<H>(cwd, &format!("git checkout -b {branch}"))?;
        Command::new("git")
            .args(["checkout", "-b", branch])
            .current_dir(cwd)
            .output()?;
    }

    Ok(())
}
