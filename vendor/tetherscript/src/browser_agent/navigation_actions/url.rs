//! URL resolution and query building for deterministic navigations.

pub(crate) fn resolve(base: &str, target: &str) -> String {
    let target = target.trim();
    if target.is_empty() {
        return base.to_string();
    }
    if absolute(target) {
        return target.to_string();
    }
    if target.starts_with('#') {
        return format!("{}{}", strip_hash(base), target);
    }
    if target.starts_with('?') {
        return format!("{}{}", strip_query_hash(base), target);
    }
    let Some((scheme, rest)) = base.split_once("://") else {
        return target.to_string();
    };
    let host_end = rest.find('/').unwrap_or(rest.len());
    let origin = format!("{}://{}", scheme, &rest[..host_end]);
    if target.starts_with('/') {
        return format!("{}{}", origin, target);
    }
    format!("{}/{}", base_dir(&origin, rest, host_end), target)
}

fn absolute(target: &str) -> bool {
    target.contains("://") || target.starts_with("data:") || target.starts_with("about:")
}

fn strip_hash(value: &str) -> &str {
    value.split('#').next().unwrap_or(value)
}

fn strip_query_hash(value: &str) -> &str {
    strip_hash(value).split('?').next().unwrap_or(value)
}

fn base_dir(origin: &str, rest: &str, host_end: usize) -> String {
    let path = strip_query_hash(rest.get(host_end..).unwrap_or(""));
    let dir = path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
    format!("{}{}", origin, dir.trim_end_matches('/'))
}
