//! HTML fragment to plain text conversion.
use regex::Regex;
use std::sync::LazyLock;

static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]+>").expect("valid tag regex"));

/// Remove tags, decode entities, and collapse whitespace.
pub(crate) fn strip_tags(html: &str) -> String {
    let text = TAG.replace_all(html, "");
    let decoded = html_escape::decode_html_entities(&text);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}
