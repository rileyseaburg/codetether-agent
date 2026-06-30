//! Parsing for the unsafe-sandbox override environment variable.

pub(in crate::tool::sandbox) fn enabled(value: Option<&str>) -> bool {
    value.is_some_and(truthy)
}

pub(super) fn allowed(env: &str) -> bool {
    enabled(std::env::var(env).ok().as_deref())
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}
