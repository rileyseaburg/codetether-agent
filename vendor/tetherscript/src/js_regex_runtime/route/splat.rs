//! Splat captures for route regexes.

pub(super) fn finish(
    text: &str,
    index: usize,
    mut captures: Vec<Option<String>>,
) -> Option<(usize, usize, Vec<Option<String>>)> {
    let tail = &text[index..];
    if tail.is_empty() || tail.chars().all(|ch| ch == '/') {
        captures.push(None);
    } else if let Some(rest) = tail.strip_prefix('/') {
        captures.push(Some(rest.into()));
    } else {
        return None;
    }
    Some((0, text.len(), captures))
}
