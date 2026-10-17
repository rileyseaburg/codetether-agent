//! Borrowed control-plane destination for a worker task release.
/// HTTP client, server and worker authentication used for release submission.
///
/// # Examples
/// ```rust
/// use codetether_a2a_http::worker::completion::ReleaseTarget;
/// let client = reqwest::Client::new();
/// let token = None;
/// let target = ReleaseTarget { client: &client, server: "http://localhost:8001", token: &token, worker_id: "worker" };
/// assert_eq!(target.worker_id, "worker");
/// ```
pub struct ReleaseTarget<'a> {
    /// HTTP client used for submission.
    pub client: &'a reqwest::Client,
    /// Control-plane base URL.
    pub server: &'a str,
    /// Optional bearer token.
    pub token: &'a Option<String>,
    /// Worker identifier for the request and payload.
    pub worker_id: &'a str,
}
