//! OAuth callback request handling.
use crate::extract::extract_oauth_code_and_state;
use anyhow::Result;

/// Extract code and state from a GET or POST callback request.
///
/// # Arguments
/// request is the captured HTTP request text.
/// # Returns
/// Code and state extracted from the query string or request body.
/// # Errors
/// Returns unsupported-method, missing-payload or callback-parsing errors.
/// # Examples
/// ```
/// let result = codetether_oauth_callback::parse_oauth_callback_request("GET /auth/callback?code=abc&state=xyz HTTP/1.1\r\n\r\n").unwrap();
/// assert_eq!(result.0, "abc");
/// ```
pub fn parse_oauth_callback_request(request: &str) -> Result<(String, String)> {
    let first_line = request
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing HTTP request line"))?;
    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap_or_default();
    let method = method.to_ascii_uppercase();

    let target = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing callback target"))?;
    let target_query = target.split_once('?').map(|(_, query)| query.trim());
    let body = request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .or_else(|| request.split_once("\n\n").map(|(_, body)| body))
        .map(str::trim)
        .filter(|body| !body.is_empty());

    let callback_payload = match method.as_str() {
        "GET" => target_query
            .or(body)
            .ok_or_else(|| anyhow::anyhow!("Callback target missing query string"))?,
        "POST" => body
            .or(target_query)
            .ok_or_else(|| anyhow::anyhow!("Callback POST body missing OAuth payload"))?,
        _ => anyhow::bail!("Unsupported callback method: {}", method),
    };

    extract_oauth_code_and_state(callback_payload)
}
