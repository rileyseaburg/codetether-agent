//! Anchored repeated character-class suffix matching.

use super::class;

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    let (body, negated) = suffix_body(pattern)?;
    let end = text.len();
    let mut start = end;
    for (index, ch) in text.char_indices().rev() {
        let matched = class::matches_class(body, ch);
        if matched == negated {
            break;
        }
        start = index;
    }
    (start < end).then_some((start, end))
}

fn suffix_body(pattern: &str) -> Option<(&str, bool)> {
    if let Some(body) = pattern
        .strip_prefix("[^")
        .and_then(|value| value.strip_suffix("]+$"))
    {
        return Some((body, true));
    }
    pattern
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix("]+$"))
        .map(|body| (body, false))
}
