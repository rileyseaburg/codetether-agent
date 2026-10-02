//! Collect removed, added, and unchanged lines from a unified-diff body.

pub(super) fn absorb(old: &mut Vec<String>, new: &mut Vec<String>, line: &str) {
    if let Some(content) = line.strip_prefix('-') {
        old.push(content.to_string());
    } else if let Some(content) = line.strip_prefix('+') {
        new.push(content.to_string());
    } else if let Some(content) = line
        .strip_prefix(' ')
        .or_else(|| line.is_empty().then_some(""))
    {
        old.push(content.to_string());
        new.push(content.to_string());
    }
}
