//! Path extraction for structured single-file and multi-file tools.

use serde_json::Value;
use std::path::PathBuf;

pub(super) fn single(input: &Value, field: &str) -> Option<Vec<PathBuf>> {
    input
        .get(field)
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .map(|path| vec![path])
}

pub(super) fn many(input: &Value) -> Option<Vec<PathBuf>> {
    let paths = input
        .get("edits")?
        .as_array()?
        .iter()
        .filter_map(|edit| edit.get("file").or_else(|| edit.get("path")))
        .filter_map(Value::as_str)
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    (!paths.is_empty()).then_some(paths)
}
