//! DuckDuckGo HTML results page request and parsing.
use crate::{SearchFailure, result::SearchResult, text::strip_tags};
use regex::Regex;
use std::sync::LazyLock;

static LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+class="result__a"[^>]+href="([^"]+)"[^>]*>(.*?)</a>"#)
        .expect("valid link regex")
});
static SNIPPET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<a[^>]+class="result__snippet"[^>]*>(.*?)</a>"#).expect("valid snippet regex")
});

pub(crate) fn url(query: &str) -> String {
    format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    )
}

/// DuckDuckGo answers blocked clients with HTTP 202 and an anomaly modal.
pub(crate) fn parse(
    status: u16,
    html: &str,
    max: usize,
) -> Result<Vec<SearchResult>, SearchFailure> {
    if html.contains("anomaly-modal") || html.contains("id=\"challenge-form\"") {
        return Err(SearchFailure::Challenge { status });
    }
    if let Some(failure) = SearchFailure::from_status(status) {
        return Err(failure);
    }
    let links: Vec<_> = LINK.captures_iter(html).collect();
    if links.is_empty() {
        return match html.contains("class=\"no-results\"") {
            true => Ok(Vec::new()),
            false => Err(SearchFailure::Malformed(
                "DuckDuckGo page has no result links".into(),
            )),
        };
    }
    let snippets: Vec<_> = SNIPPET.captures_iter(html).collect();
    let results = links
        .iter()
        .take(max)
        .enumerate()
        .map(|(i, cap)| SearchResult {
            title: strip_tags(&cap[2]),
            url: unwrap(&cap[1]),
            snippet: snippets
                .get(i)
                .map(|c| strip_tags(&c[1]))
                .unwrap_or_default(),
        });
    Ok(results.collect())
}

fn unwrap(href: &str) -> String {
    let href = html_escape::decode_html_entities(href);
    href.split("uddg=")
        .nth(1)
        .and_then(|s| urlencoding::decode(s.split('&').next().unwrap_or("")).ok())
        .map(|s| s.into_owned())
        .unwrap_or_else(|| href.into_owned())
}
