//! Application verification entrypoint with a policy-gated shell adapter.
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_shell;
use anyhow::Result;
use codetether_application_registry::Application;
use codetether_ralph_core::UserStory;
use std::path::Path;
/// Run all story checks through the application's policy-gated shell.
/// # Arguments
/// * `root` - Base directory for story checks.
/// * `story` - Story with verification steps.
/// # Returns
/// Unit when every check succeeds.
/// # Errors
/// Returns the shared verifier's aggregated step failures.
/// # Examples
/// `run_story_verification::<T>(root, story).await` uses tool application `T`.
pub async fn run_story_verification<T: Application>(root: &Path, story: &UserStory) -> Result<()> {
    codetether_ralph_core::verification::run_story_verification(
        root,
        story,
        &codetether_ralph_agent::shell::Verification::<T>::new(),
    )
    .await
}

#[cfg(test)]
mod test_application;
