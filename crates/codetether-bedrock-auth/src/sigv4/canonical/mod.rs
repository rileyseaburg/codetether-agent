//! Canonical URL components used by the SigV4 signer.
use anyhow::{Context, Result};
use reqwest::Url;
mod path;
mod query;
use path::{canonical_host, canonical_uri};
use query::canonical_querystring;
/// Canonical host, path, and sorted encoded query used in an AWS signature.
///
/// # Examples
/// ```
/// let value = codetether_bedrock_auth::sigv4::canonicalize_url("https://example.com/a?b=2")?;
/// assert_eq!(value.canonical_uri, "/a");
/// # Ok::<(), anyhow::Error>(())
/// ```

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalUrl {
    /// Canonical host included in the signed request.
    pub host: String,
    /// Canonical canonical uri included in the signed request.
    pub canonical_uri: String,
    /// Canonical canonical querystring included in the signed request.
    pub canonical_querystring: String,
}
/// Canonicalize the host, path, and query used for signing.
///
/// # Arguments
/// * `url` - Absolute request URL.
///
/// # Returns
/// Encoded path and sorted query components.
///
/// # Errors
/// Returns an error if the URL, host, path, or percent encoding is invalid.
///
/// # Examples
/// See [CanonicalUrl].
pub fn canonicalize_url(url: &str) -> Result<CanonicalUrl> {
    let parsed = Url::parse(url).with_context(|| format!("Invalid Bedrock URL: {url}"))?;
    let host = canonical_host(&parsed).context("Bedrock URL missing host")?;
    let canonical_uri = canonical_uri(&parsed)?;
    let canonical_querystring = canonical_querystring(&parsed);
    Ok(CanonicalUrl {
        host,
        canonical_uri,
        canonical_querystring,
    })
}
