//! Best-effort publication of one result per benchmarked model.
use crate::BenchmarkSuiteResult;
use tracing::warn;
/// Submit each model result, logging failures and continuing to the next model.
///
/// # Arguments
/// * `result` - Completed suite to serialize and publish.
/// * `api_url` - Submission endpoint.
/// * `api_key` - Bearer token for that endpoint.
/// # Returns
/// Completes after all submissions, or returns early if setup fails.
/// # Examples
/// ```no_run
/// # async fn example(result: &codetether_benchmark::BenchmarkSuiteResult) {
/// codetether_benchmark::http::submit_results(result, "https://example.com/results", "token").await;
/// # }
/// ```
pub async fn submit_results(result: &BenchmarkSuiteResult, api_url: &str, api_key: &str) {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to create HTTP client for submission: {}", e);
            return;
        }
    };

    let result_json = match serde_json::to_string(result) {
        Ok(j) => j,
        Err(e) => {
            warn!("Failed to serialize results: {}", e);
            return;
        }
    };

    for mr in &result.model_results {
        let submission = crate::BenchmarkSubmission {
            model: mr.model.clone(),
            agent: format!("{} v{}", result.agent, result.agent_version),
            result: result_json.clone(),
        };

        super::send_submission::send_submission(&client, api_url, api_key, &submission).await;
    }
}
