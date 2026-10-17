//! GitHub pull-request response fields consumed by callers.
use super::*;

#[derive(Debug, Deserialize)]
/// Fields returned by PR creation, updates, and branch lookup.
///
/// # Examples
/// ```
/// let response: codetether_github_pr::http::PullRequestResponse = serde_json::from_value(
///     serde_json::json!({"number":1,"html_url":"url","state":"open","draft":true}),
/// ).unwrap();
/// assert_eq!(response.number, 1);
/// ```
pub struct PullRequestResponse {
    /// Repository-local PR number.
    pub number: u64,
    /// Browser URL returned by GitHub.
    pub html_url: String,
    /// API state string.
    pub state: String,
    /// Whether the returned PR is a draft.
    pub draft: bool,
}
