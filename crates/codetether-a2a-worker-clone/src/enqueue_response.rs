//! Validate the post-clone task enqueue response.
use anyhow::Result;
pub(super) async fn require_success(response: reqwest::Response) -> Result<()> {
    if response.status().is_success() {
        return Ok(());
    }
    anyhow::bail!(
        "Failed to enqueue post-clone task ({}): {}",
        response.status(),
        response.text().await.unwrap_or_default()
    )
}
