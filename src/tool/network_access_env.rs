//! Process-default network authority.

pub(crate) fn allowed() -> bool {
    let primary = std::env::var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK").ok();
    let fallback = std::env::var("CODETETHER_ALLOW_NETWORK").ok();
    resolve(primary.as_deref(), fallback.as_deref())
}

pub(super) fn resolve(primary: Option<&str>, fallback: Option<&str>) -> bool {
    primary.or(fallback).is_some_and(truthy)
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}