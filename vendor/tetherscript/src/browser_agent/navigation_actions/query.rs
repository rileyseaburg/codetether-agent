//! Query string assembly for GET form navigations.

pub(crate) fn with_query(url: &str, entries: &[(String, String)]) -> String {
    if entries.is_empty() {
        return url.to_string();
    }
    let (base, hash) = url.split_once('#').unwrap_or((url, ""));
    let separator = if base.contains('?') { "&" } else { "?" };
    let query = super::encode::pairs(entries);
    if hash.is_empty() {
        format!("{base}{separator}{query}")
    } else {
        format!("{base}{separator}{query}#{hash}")
    }
}
