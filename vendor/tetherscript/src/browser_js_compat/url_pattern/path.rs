use super::*;

pub(super) fn parts(raw: &str, pattern: bool) -> model::Parts {
    let (pathname, search, hash) = split(raw);
    let mut parts = model::Parts::any();
    parts.pathname = norm::pathname(pathname);
    parts.search = optional(norm::search(search), pattern);
    parts.hash = optional(norm::hash(hash), pattern);
    parts
}

fn split(raw: &str) -> (&str, &str, &str) {
    let (before_hash, hash) = raw.split_once('#').map_or((raw, ""), |(a, b)| (a, b));
    let (pathname, search) = before_hash
        .split_once('?')
        .map_or((before_hash, ""), |(a, b)| (a, b));
    (pathname, search, hash)
}

fn optional(value: String, pattern: bool) -> String {
    if pattern && value.is_empty() {
        "*".into()
    } else {
        value
    }
}
