//! Canonical host and individually encoded path segments.
use anyhow::{Context, Result};
use reqwest::Url;
pub(super) fn canonical_host(url: &Url) -> Option<String> {
    let host = url.host_str()?.to_string();
    Some(match url.port() {
        Some(port) => format!("{host}:{port}"),
        None => host,
    })
}
pub(super) fn canonical_uri(url: &Url) -> Result<String> {
    let segments = url.path_segments().context("Bedrock URL missing path")?;
    let encoded = segments
        .map(canonical_path_segment)
        .collect::<Result<Vec<_>>>()?;
    Ok(format!("/{}", encoded.join("/")))
}
pub(super) fn canonical_path_segment(segment: &str) -> Result<String> {
    let decoded = urlencoding::decode(segment)
        .with_context(|| format!("invalid percent-encoding in path segment `{segment}`"))?;
    Ok(urlencoding::encode(&decoded).into_owned())
}
