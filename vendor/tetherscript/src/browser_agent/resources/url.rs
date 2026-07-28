//! Small URL resolution helper for deterministic resources.

pub(crate) fn resolve(base: &str, reference: &str) -> String {
    if has_scheme(reference) || reference.starts_with("data:") {
        return reference.into();
    }
    if reference.starts_with('/') {
        return origin(base)
            .map(|origin| super::url_norm::clean(format!("{}{}", origin, reference)))
            .unwrap_or_else(|| reference.into());
    }
    let prefix = base.rsplit_once('/').map_or(base, |(prefix, _)| prefix);
    if prefix.is_empty() {
        super::url_norm::clean(reference.into())
    } else {
        super::url_norm::clean(format!("{}/{}", prefix.trim_end_matches('/'), reference))
    }
}

pub(crate) fn candidates(base: &str, reference: &str) -> Vec<String> {
    let resolved = resolve(base, reference);
    let mut out = vec![reference.into(), resolved.clone()];
    if let Some(path) = path_only(&resolved) {
        out.push(path.into());
    }
    out
}

fn has_scheme(value: &str) -> bool {
    value
        .split_once(':')
        .is_some_and(|(scheme, _)| scheme.chars().all(|ch| ch.is_ascii_alphabetic()))
}

fn origin(url: &str) -> Option<&str> {
    let scheme_end = url.find("://")? + 3;
    let rest = &url[scheme_end..];
    let host_len = rest.find('/').unwrap_or(rest.len());
    Some(&url[..scheme_end + host_len])
}

fn path_only(url: &str) -> Option<&str> {
    let scheme_end = url.find("://")? + 3;
    let rest = &url[scheme_end..];
    let path = rest.find('/')?;
    Some(&rest[path..])
}
