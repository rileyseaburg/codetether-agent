use super::*;

pub(super) fn parts(input: &str, base: Option<&str>, pattern: bool) -> model::Parts {
    let raw = input.trim();
    if base.is_none() && !raw.contains("://") {
        return path::parts(raw, pattern);
    }
    from_href(&resolve_url(raw, base), pattern)
}

fn from_href(href: &str, pattern: bool) -> model::Parts {
    let parsed = parse_location(href);
    model::Parts {
        protocol: norm::protocol(&field(&parsed, "protocol")),
        hostname: norm::hostname(&field(&parsed, "hostname")),
        pathname: norm::pathname(&field(&parsed, "pathname")),
        search: optional(norm::search(&field(&parsed, "search")), pattern),
        hash: optional(norm::hash(&field(&parsed, "hash")), pattern),
    }
}

fn field(parsed: &HashMap<String, JsValue>, name: &str) -> String {
    parsed.get(name).map(JsValue::display).unwrap_or_default()
}

fn optional(value: String, pattern: bool) -> String {
    if pattern && value.is_empty() {
        "*".into()
    } else {
        value
    }
}
