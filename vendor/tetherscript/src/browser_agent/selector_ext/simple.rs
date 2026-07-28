//! Simple selector extension pseudo matching.

use super::types::SelectorFilter;
use super::types::SelectorFilter::*;

pub(crate) fn parse_simple(
    source: &str,
    index: usize,
    filters: &mut Vec<SelectorFilter>,
) -> Option<usize> {
    for (name, filter) in [
        (":visible", Visible),
        (":enabled", Enabled),
        (":disabled", Disabled),
        (":checked", Checked),
    ] {
        if let Some(end) = simple(source, index, name) {
            filters.push(filter);
            return Some(end);
        }
    }
    None
}

fn simple(source: &str, index: usize, name: &str) -> Option<usize> {
    if !source[index..].starts_with(name) {
        return None;
    }
    let end = index + name.len();
    boundary(source, end).then_some(end)
}

fn boundary(source: &str, index: usize) -> bool {
    source[index..]
        .chars()
        .next()
        .is_none_or(|ch| !matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
}
