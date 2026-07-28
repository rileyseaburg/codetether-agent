//! Request URL normalization for security metadata.

use super::url::parse_origin;

pub fn resolve_url(base: &str, target: &str) -> String {
    let target = target.trim();
    if target.contains("://") {
        return target.into();
    }
    let base_origin = parse_origin(base).serialized();
    if let Some(rest) = target.strip_prefix("//") {
        let scheme = base.split_once("://").map_or("https", |(scheme, _)| scheme);
        return format!("{scheme}://{rest}");
    }
    if target.starts_with('/') {
        return format!("{base_origin}{target}");
    }
    format!("{base_origin}/{target}")
}

pub fn referrer_for(url: &str) -> Option<String> {
    let value = url.split('#').next().unwrap_or(url).trim();
    (!value.is_empty()).then(|| value.into())
}
