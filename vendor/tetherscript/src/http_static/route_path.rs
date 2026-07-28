//! Path joins for capability paths and URL routes.

/// Join capability-relative path segments with `/`.
pub(crate) fn join_path(parent: &str, name: &str) -> String {
    format!("{}/{}", parent.trim_end_matches('/'), name)
}

/// Join URL route path segments with `/`.
pub(crate) fn join_route(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        format!("/{}", name)
    } else {
        format!("{}/{}", parent, name)
    }
}
