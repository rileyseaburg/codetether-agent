//! OAuth callback extract handling.
use crate::query::parse_query_pairs;
use anyhow::{Context, Result};

/// Parse an OAuth code and state from a pasted callback URL or query.
///
/// # Arguments
/// callback_input is a URL, URL path with query, or raw query string.
/// # Returns
/// Nonempty code and state values; state matching belongs to the caller.
/// # Errors
/// Returns malformed-URL, OAuth error-response or missing-value errors.
/// # Examples
/// ```
/// let (code, state) = codetether_oauth_callback::extract_oauth_code_and_state("code=abc&state=xyz").unwrap();
/// assert_eq!((code.as_str(), state.as_str()), ("abc", "xyz"));
/// ```
pub fn extract_oauth_code_and_state(callback_input: &str) -> Result<(String, String)> {
    let input = callback_input.trim();
    if input.is_empty() {
        anyhow::bail!("Callback URL is required");
    }

    let query = if input.contains("://") {
        let url =
            reqwest::Url::parse(input).with_context(|| format!("Invalid callback URL: {input}"))?;
        url.query()
            .map(str::to_string)
            .ok_or_else(|| anyhow::anyhow!("Callback URL is missing query parameters"))?
    } else if let Some((_, params)) = input.split_once('?') {
        params.to_string()
    } else {
        input.to_string()
    };

    let params = parse_query_pairs(&query);
    if let Some(error) = params.get("error") {
        let error_description = params
            .get("error_description")
            .map(String::as_str)
            .unwrap_or("No error description provided");
        anyhow::bail!(
            "OAuth authorization failed: {} ({})",
            error,
            error_description
        );
    }

    let code = params
        .get("code")
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Callback URL does not include an OAuth code"))?;
    let state = params
        .get("state")
        .cloned()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Callback URL does not include OAuth state"))?;

    Ok((code, state))
}
