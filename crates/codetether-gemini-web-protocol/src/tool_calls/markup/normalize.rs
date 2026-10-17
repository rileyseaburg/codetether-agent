//! Canonicalization limited to protocol tags rather than argument payloads.

use regex::Regex;
use std::sync::OnceLock;

pub(super) fn text(input: &str) -> String {
    let html = html_regex().replace_all(input, "<${1}tool_${2}>");
    let escaped = escaped_regex().replace_all(&html, "<${1}tool_${2}>");
    plain_regex()
        .replace_all(&escaped, "<${1}tool_${2}>")
        .replace("\\_", "_")
}

fn html_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| Regex::new(r"(?i)&lt;\s*(/?)\s*tool(?:_|-)?(call|result)\s*&gt;").unwrap())
}

fn escaped_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE
        .get_or_init(|| Regex::new(r"(?i)\\<\s*(/?)\s*tool(?:\\?_|-)?(call|result)\s*\\>").unwrap())
}

fn plain_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| Regex::new(r"(?i)<\s*(/?)\s*tool(?:_|-)?(call|result)\s*>").unwrap())
}
