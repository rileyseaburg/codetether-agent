//! Sorted percent-encoded query parameters.
use reqwest::Url;
pub(super) fn canonical_querystring(url: &Url) -> String {
    let mut pairs = url
        .query_pairs()
        .map(|(key, value)| {
            (
                urlencoding::encode(&key).into_owned(),
                urlencoding::encode(&value).into_owned(),
            )
        })
        .collect::<Vec<_>>();
    pairs.sort();
    pairs
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}
