//! Resolve a redirect target relative to its source URL.
pub(super) fn resolve(base: &str, location: &str) -> String {
    reqwest::Url::parse(base)
        .and_then(|b| b.join(location))
        .map(|u| u.to_string())
        .unwrap_or_else(|_| location.to_string())
}
