//! JavaScript escaping helpers for keyboard action snippets.

pub(crate) fn node(path: &[usize]) -> String {
    let mut script = String::from("document");
    for index in path {
        script.push_str(&format!(".childNodes[{}]", index));
    }
    script
}

pub(crate) fn quote(value: &str) -> String {
    let mut out = String::from("'");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('\'');
    out
}
