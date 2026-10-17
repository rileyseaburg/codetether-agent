//! Story-level verification with host-provided shell execution.
//!
//! run_story_verification executes all declared checks and aggregates failures.
//! Implement ShellExecutor to supply the host's command policy and tool runtime.
mod executor;
pub use executor::{ShellExecutor, ShellOutput};
mod file;
mod shell;
mod step;
mod url;

use crate::UserStory;
use anyhow::{Result, anyhow};
use std::path::Path;

/// Run every story verification step and aggregate failures in step order.
///
/// # Arguments
/// * `root` - Working directory used for relative paths.
/// * `story` - Story whose verification steps are checked.
/// * `executor` - Host shell executor, including its policy gate.
/// # Returns
/// Unit if all steps pass, including when no steps are declared.
/// # Errors
/// Returns all step failures joined by newlines, with one-based step numbers.
/// # Examples
/// ```no_run
/// # async fn example(story: &codetether_ralph_core::UserStory, executor: &dyn codetether_ralph_core::verification::ShellExecutor) -> anyhow::Result<()> {
/// codetether_ralph_core::verification::run_story_verification(std::path::Path::new("."), story, executor).await?;
/// # Ok(()) }
/// ```
pub async fn run_story_verification(
    root: &Path,
    story: &UserStory,
    executor: &dyn ShellExecutor,
) -> Result<()> {
    let mut failures = Vec::new();
    let client = reqwest::Client::new();
    for (index, item) in story.verification_steps.iter().enumerate() {
        if let Err(error) = step::run(root, item, &client, executor).await {
            failures.push(format!("step {}: {error}", index + 1));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(failures.join("\n")))
    }
}
