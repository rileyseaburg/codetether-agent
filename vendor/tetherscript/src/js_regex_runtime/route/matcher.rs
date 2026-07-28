//! Route regex body matcher.

use super::{literal, segment};

pub(super) fn matches(
    text: &str,
    mut pattern: &str,
    insensitive: bool,
    index: &mut usize,
    captures: &mut Vec<Option<String>>,
) -> bool {
    while !pattern.is_empty() {
        if let Some(rest) = pattern.strip_prefix(r"([^\/]+)") {
            if !segment::required(text, index, captures) {
                return false;
            }
            pattern = rest;
        } else if let Some(rest) = pattern.strip_prefix(r"/?([^\/]+)?") {
            segment::optional(text, index, captures);
            pattern = rest;
        } else {
            let (expected, rest) = literal::read(pattern);
            if !literal::take(text, index, expected, insensitive) {
                return false;
            }
            pattern = rest;
        }
    }
    true
}
