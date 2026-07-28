//! Dot-segment normalization for deterministic resource URLs.

pub(crate) fn clean(url: String) -> String {
    let Some(scheme_end) = url.find("://").map(|index| index + 3) else {
        return clean_path(&url);
    };
    let rest = &url[scheme_end..];
    let host_len = rest.find('/').unwrap_or(rest.len());
    let path_start = scheme_end + host_len;
    let (prefix, path) = url.split_at(path_start);
    format!("{}{}", prefix, clean_path(path))
}

fn clean_path(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut parts = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            value => parts.push(value),
        }
    }
    let joined = parts.join("/");
    if absolute {
        format!("/{}", joined)
    } else {
        joined
    }
}
