//! Application verification entrypoint with a policy-gated shell adapter.
mod shell;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_shell;
use super::types::UserStory;
use anyhow::Result;
use std::path::Path;
pub async fn run_story_verification(root: &Path, story: &UserStory) -> Result<()> {
    codetether_ralph_core::verification::run_story_verification(root, story, &shell::HostShell)
        .await
}
