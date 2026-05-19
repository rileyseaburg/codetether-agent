//! JSON-extraction helpers shared by [`super::arg_preview`].

use serde_json::Value;

pub(super) fn string_field(v: &Value, key: &str) -> String {
    str_or(v, key)
}

pub(super) fn str_or(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

pub(super) fn format_read_file(v: &Value) -> String {
    let path = string_field(v, "filePath");
    match (v.get("startLine"), v.get("endLine")) {
        (Some(s), Some(e)) => format!("{path} :{s}-{e}"),
        _ => path,
    }
}

pub(super) fn format_write(v: &Value) -> String {
    let path = string_field(v, "filePath");
    let len = v
        .get("content")
        .and_then(Value::as_str)
        .map(str::len)
        .unwrap_or(0);
    format!("📝 {path} ({len} bytes)")
}

pub(super) fn format_k8s(v: &Value) -> String {
    for k in ["name", "resourceType", "namespace", "kind"] {
        let s = str_or(v, k);
        if !s.is_empty() {
            return format!("{k}={s}");
        }
    }
    serde_json::to_string(v).unwrap_or_default()
}
