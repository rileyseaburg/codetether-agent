const PATTERN: &str = r"^%?[^%]*%?$";

pub(super) fn find(text: &str, pattern: &str) -> Option<(usize, usize)> {
    (pattern == PATTERN && valid(text)).then_some((0, text.len()))
}

fn valid(text: &str) -> bool {
    let inner = text.strip_prefix('%').unwrap_or(text);
    let inner = inner.strip_suffix('%').unwrap_or(inner);
    !inner.contains('%')
}
