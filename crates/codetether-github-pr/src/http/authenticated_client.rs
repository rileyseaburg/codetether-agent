//! Construct the authenticated GitHub HTTP client.
use super::*;
/// Build the standard GitHub headers from a caller-provided token.
///
/// # Arguments
/// * `token` - Bearer credential obtained by the caller.
/// # Returns
/// A client with the existing API version, Accept, and User-Agent headers.
/// # Errors
/// Returns an error for an invalid header value or HTTP client configuration.
/// # Examples
/// ```no_run
/// # fn example(token: &str) -> anyhow::Result<()> {
/// let client = codetether_github_pr::http::authenticated_client(token)?;
/// let _ = client;
/// # Ok(()) }
/// ```
pub fn authenticated_client(token: &str) -> Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", token).parse()?,
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/vnd.github+json".parse()?,
    );
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse()?);
    headers.insert(reqwest::header::USER_AGENT, "codetether-pr-helper".parse()?);
    Ok(Client::builder().default_headers(headers).build()?)
}
