//! Selector generation helpers.

use super::super::DomNode;

pub fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn fragile_id(s: &str) -> bool {
    s.chars().filter(|c| c.is_ascii_digit()).count() > 4 || s.len() > 32
}

pub fn unique_class(n: &DomNode) -> Option<String> {
    n.attr("class")?
        .split_whitespace()
        .find(|c| !c.contains('_') && !c.chars().any(|x| x.is_ascii_digit()))
        .map(str::to_string)
}
