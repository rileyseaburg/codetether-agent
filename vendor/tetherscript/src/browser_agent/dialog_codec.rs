//! Tiny escaping helpers for dialog records stored in session metadata.

pub(crate) fn escape(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out
}

pub(crate) fn unescape(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

pub(crate) fn opt(value: &Option<String>) -> String {
    value
        .as_ref()
        .map(|value| format!("+{}", escape(value)))
        .unwrap_or_else(|| "-".into())
}

pub(crate) fn parse_opt(value: &str) -> Option<String> {
    value.strip_prefix('+').map(unescape)
}
