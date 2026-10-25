//! Self-hosted SearXNG JSON API request and parsing.
//!
//! The instance must enable the `json` output format
//! (`search.formats: [html, json]`); otherwise SearXNG answers 403, which is
//! reported as [`SearchFailure::Forbidden`].
use crate::{SearchFailure, result::SearchResult, text::strip_tags};
use serde::Deserialize;

/// Environment variable holding the SearXNG base URL, e.g.
/// `http://searxng.searxng.svc.cluster.local:8080`.
pub const SEARXNG_URL_ENV: &str = "CODETETHER_SEARXNG_URL";

#[derive(Deserialize)]
struct Response {
    #[serde(default)]
    results: Vec<Item>,
    #[serde(default)]
    unresponsive_engines: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct Item {
    #[serde(default)]
    title: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    content: String,
}

/// Configured instance base URL, if [`SEARXNG_URL_ENV`] is set and non-empty.
pub(crate) fn configured() -> Option<String> {
    let value = std::env::var(SEARXNG_URL_ENV).ok()?;
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

/// Base URL or a config failure when SearXNG was selected but not configured.
pub(crate) fn base_url() -> Result<String, SearchFailure> {
    configured().ok_or_else(|| SearchFailure::Config(format!("{SEARXNG_URL_ENV} is not set")))
}

/// JSON search URL for `query` against the instance at `base`.
pub(crate) fn url(base: &str, query: &str) -> String {
    format!(
        "{}/search?q={}&format=json&language=en-US",
        base.trim_end_matches('/'),
        urlencoding::encode(query)
    )
}

/// Empty results with every engine unresponsive is an upstream outage, not
/// "no results".
pub(crate) fn parse(
    status: u16,
    body: &str,
    max: usize,
) -> Result<Vec<SearchResult>, SearchFailure> {
    if let Some(failure) = SearchFailure::from_status(status) {
        return Err(failure);
    }
    let page: Response = serde_json::from_str(body)
        .map_err(|e| SearchFailure::Malformed(format!("SearXNG JSON: {e}")))?;
    if page.results.is_empty() && !page.unresponsive_engines.is_empty() {
        return Err(SearchFailure::Upstream(format!(
            "SearXNG engines unresponsive: {}",
            serde_json::Value::from(page.unresponsive_engines)
        )));
    }
    let items = page
        .results
        .into_iter()
        .filter(|i| i.url.starts_with("http"));
    let results = items.take(max).map(|i| SearchResult {
        title: strip_tags(&i.title),
        url: i.url,
        snippet: strip_tags(&i.content),
    });
    Ok(results.collect())
}
