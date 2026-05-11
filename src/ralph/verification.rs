mod file;
mod shell;
mod step;
#[cfg(test)]
mod tests;
mod url;

use super::types::UserStory;
use anyhow::{Result, anyhow};
use std::path::Path;

pub async fn run_story_verification(root: &Path, story: &UserStory) -> Result<()> {
    let mut failures = Vec::new();
    let client = crate::provider::shared_http::shared_client();
    for (index, item) in story.verification_steps.iter().enumerate() {
        if let Err(error) = step::run(root, item, &client).await {
            failures.push(format!("step {}: {error}", index + 1));
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(failures.join("\n")))
    }
}
