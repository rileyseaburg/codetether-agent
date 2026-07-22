//! Normalization and recognition of Gemini's XML-like markup.

mod normalize;

use regex::Regex;
use std::sync::OnceLock;

pub(super) fn normalize(input: &str) -> String {
    normalize::text(input)
}

pub(super) fn call_start_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| {
        Regex::new(r"(?is)(?:```(?:xml|json)?\s*)?<tool_call\s*>\s*(?:```(?:json)?\s*)?").unwrap()
    })
}

pub(super) fn call_close_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| Regex::new(r"(?is)^</tool_call\s*>").unwrap())
}

pub(super) fn call_open_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| Regex::new(r"(?is)^<tool_call\s*>").unwrap())
}

pub(super) fn contains_call_opener(input: &str) -> bool {
    normalize(input).to_ascii_lowercase().contains("<tool_call")
}

pub(super) fn contains_result_opener(input: &str) -> bool {
    normalize(input)
        .to_ascii_lowercase()
        .contains("<tool_result")
}

pub(super) fn result_regex() -> &'static Regex {
    static VALUE: OnceLock<Regex> = OnceLock::new();
    VALUE.get_or_init(|| Regex::new(r"(?is)<tool_result\s*>.*?</?tool_result\s*>").unwrap())
}
