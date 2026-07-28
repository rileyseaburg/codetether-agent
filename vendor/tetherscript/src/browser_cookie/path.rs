//! Cookie path helpers.

pub(crate) fn path_for_url(url: &str) -> String {
    let Some((_, rest)) = url.split_once("://") else {
        return "/".into();
    };
    let path_start = rest.find('/').unwrap_or(rest.len());
    let path = rest[path_start..].split(['?', '#']).next().unwrap_or("/");
    if path.is_empty() {
        "/".into()
    } else {
        path.into()
    }
}

pub(crate) fn default_cookie_path(url: &str) -> String {
    let path = path_for_url(url);
    if !path.starts_with('/') || path.matches('/').count() <= 1 {
        return "/".into();
    }
    path.rsplit_once('/')
        .map(|(prefix, _)| prefix)
        .filter(|prefix| !prefix.is_empty())
        .unwrap_or("/")
        .to_string()
}

pub(crate) fn path_matches(cookie_path: &str, request_path: &str) -> bool {
    if request_path == cookie_path {
        return true;
    }
    request_path.starts_with(cookie_path)
        && (cookie_path.ends_with('/')
            || request_path
                .as_bytes()
                .get(cookie_path.len())
                .is_some_and(|byte| *byte == b'/'))
}
