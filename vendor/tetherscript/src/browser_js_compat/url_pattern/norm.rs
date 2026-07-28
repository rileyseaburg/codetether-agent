pub(super) fn protocol(value: &str) -> String {
    let value = value.trim().trim_end_matches(':');
    wildcard_or(value, || value.to_ascii_lowercase())
}

pub(super) fn hostname(value: &str) -> String {
    let value = value.trim();
    wildcard_or(value, || value.to_ascii_lowercase())
}

pub(super) fn pathname(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() || value == "*" || value.starts_with('/') {
        value.into()
    } else {
        format!("/{value}")
    }
}

pub(super) fn search(value: &str) -> String {
    prefixed(value, '?')
}

pub(super) fn hash(value: &str) -> String {
    prefixed(value, '#')
}

fn prefixed(value: &str, prefix: char) -> String {
    let value = value.trim();
    if value == "*" {
        "*".into()
    } else {
        value.trim_start_matches(prefix).into()
    }
}

fn wildcard_or(value: &str, build: impl FnOnce() -> String) -> String {
    if value == "*" {
        "*".into()
    } else {
        build()
    }
}
