//! Normalized membership checks for the introduction ledger.

/// Whether `identity` was already introduced (normalized, no trailing `/`).
///
/// # Arguments
/// * `identity` - Endpoint or capability-qualified peer identity.
/// * `path` - Current ledger path, if storage is configured.
/// # Returns
/// Whether the normalized identity is present; read errors count as absent.
/// # Examples
/// ```rust
/// use codetether_a2a_http::intro::ledger::contains;
/// assert!(!contains("http://peer/", None));
/// ```
pub fn contains(identity: &str, path: Option<std::path::PathBuf>) -> bool {
    super::load(path)
        .unwrap_or_default()
        .contains(identity.trim_end_matches('/'))
}
