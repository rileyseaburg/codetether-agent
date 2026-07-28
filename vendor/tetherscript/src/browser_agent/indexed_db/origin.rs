//! Origin key normalization for IndexedDB buckets.

pub(crate) fn indexed_db_origin(input: &str) -> String {
    let Some((scheme, rest)) = input.split_once("://") else {
        return input.split('#').next().unwrap_or(input).to_string();
    };
    let authority = rest
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    format!("{}://{}", scheme.to_ascii_lowercase(), authority)
}
