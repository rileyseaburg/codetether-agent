//! Validate canonical Forgejo host, account, and slot components.
pub(super) fn normalize_host(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    let value = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"))
        .unwrap_or(&value);
    let host = value.split('/').next().unwrap_or_default();
    valid(host, ".-:").then(|| host.to_string())
}
pub(super) fn normalize_component(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    valid(&value, "._-").then_some(value)
}
fn valid(value: &str, extra: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphanumeric())
        && value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || extra.contains(ch))
}
