use super::*;

pub(super) fn matches(pattern: &model::Parts, target: &model::Parts) -> bool {
    same_folded(&pattern.protocol, &target.protocol)
        && same_folded(&pattern.hostname, &target.hostname)
        && glob::matches(&pattern.pathname, &target.pathname)
        && glob::matches(&pattern.search, &target.search)
        && glob::matches(&pattern.hash, &target.hash)
}

fn same_folded(pattern: &str, value: &str) -> bool {
    glob::matches(&pattern.to_ascii_lowercase(), &value.to_ascii_lowercase())
}
