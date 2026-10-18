//! JSON path-field resolution for diff rendering.

pub(super) fn file_field(v: &serde_json::Value) -> String {
    str_field(v, "path", "filePath")
        .or_else(|| v.get("file").and_then(|x| x.as_str()))
        .unwrap_or("unknown")
        .to_string()
}

/// Read a string field accepting either a snake_case or camelCase key.
pub(super) fn str_field<'a>(v: &'a serde_json::Value, snake: &str, camel: &str) -> Option<&'a str> {
    v.get(snake)
        .or_else(|| v.get(camel))
        .and_then(|x| x.as_str())
}
