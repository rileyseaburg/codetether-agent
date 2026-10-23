//! Bing HTML results page request and parsing.
use crate::{SearchFailure, result::SearchResult, text::strip_tags};
use base64::Engine;
use regex::Regex;
use std::sync::LazyLock;

static TITLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<h2[^>]*>\s*<a\b[^>]*?href="([^"]+)"[^>]*>(.*?)</a>"#)
        .expect("valid title regex")
});
static SNIPPET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?s)<p class="b_lineclamp[^"]*"[^>]*>(.*?)</p>"#).expect("valid snippet regex")
});

pub(crate) fn url(query: &str) -> String {
    format!(
        "https://www.bing.com/search?q={}&setlang=en-US",
        urlencoding::encode(query)
    )
}

/// A page without the `b_results` list is never treated as zero results.
pub(crate) fn parse(
    status: u16,
    html: &str,
    max: usize,
) -> Result<Vec<SearchResult>, SearchFailure> {
    let listed = html.contains("id=\"b_results\"");
    if !listed && challenge(html) {
        return Err(SearchFailure::Challenge { status });
    }
    if let Some(failure) = SearchFailure::from_status(status) {
        return Err(failure);
    }
    if !listed {
        return Err(SearchFailure::Malformed(
            "Bing page has no b_results list".into(),
        ));
    }
    let blocks = html.split("<li class=\"b_algo\"").skip(1);
    Ok(blocks.filter_map(block).take(max).collect())
}

fn challenge(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    lower.contains("captcha") || lower.contains("verify you are a human")
}

fn block(chunk: &str) -> Option<SearchResult> {
    let cap = TITLE.captures(chunk)?;
    let url = resolve(&cap[1])?;
    let snippet = SNIPPET
        .captures(chunk)
        .map(|c| strip_tags(&c[1]))
        .unwrap_or_default();
    Some(SearchResult {
        title: strip_tags(&cap[2]),
        url,
        snippet,
    })
}

/// Unwrap `bing.com/ck/a?...&u=a1<base64url>` tracking links to the target URL.
pub(crate) fn resolve(href: &str) -> Option<String> {
    let href = html_escape::decode_html_entities(href);
    if !href.starts_with("https://www.bing.com/ck/a") {
        return href.starts_with("http").then(|| href.into_owned());
    }
    let encoded = href.split("&u=a1").nth(1)?.split('&').next()?;
    let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
    let bytes = engine.decode(encoded.trim_end_matches('=')).ok()?;
    String::from_utf8(bytes)
        .ok()
        .filter(|u| u.starts_with("http"))
}
