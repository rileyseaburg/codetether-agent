//! Send one already-serialized benchmark submission.
use crate::BenchmarkSubmission;
use tracing::{info, warn};
pub(super) async fn send_submission(
    client: &reqwest::Client,
    api_url: &str,
    api_key: &str,
    submission: &BenchmarkSubmission,
) {
    match client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(submission)
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            info!("Submitted benchmark results for {}", submission.model);
        }
        Ok(resp) => {
            warn!(
                "Benchmark submission failed for {} (HTTP {})",
                submission.model,
                resp.status()
            );
        }
        Err(e) => {
            warn!(
                "Failed to submit benchmark results for {}: {}",
                submission.model, e
            );
        }
    }
}
