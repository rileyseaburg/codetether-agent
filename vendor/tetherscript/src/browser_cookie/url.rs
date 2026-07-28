//! URL helpers for cookie scoping.

pub(crate) fn storage_origin(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else {
        return url.split('#').next().unwrap_or(url).to_string();
    };
    let authority = rest
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    format!("{}://{}", scheme.to_ascii_lowercase(), authority)
}

pub(crate) fn host_for_url(url: &str) -> String {
    let Some((_, rest)) = url.split_once("://") else {
        return String::new();
    };
    rest.split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .split(':')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
}

pub(crate) fn is_https(url: &str) -> bool {
    url.to_ascii_lowercase().starts_with("https://")
}

pub(crate) fn domain_matches(host: &str, domain: &str) -> bool {
    host == domain || host.ends_with(&format!(".{domain}"))
}

pub(crate) fn same_site(left: &str, right: &str) -> bool {
    let left = host_for_url(left);
    let right = host_for_url(right);
    left == right || domain_matches(&left, &right) || domain_matches(&right, &left)
}
