//! Extract edit result metadata paths without accepting non-string values.
use serde_json::Value;
use std::collections::HashMap;

pub(super) fn collect_metadata_paths(
    files: &mut Vec<String>,
    tool_metadata: Option<&HashMap<String, Value>>,
    keys: &[&str],
) {
    let Some(tool_metadata) = tool_metadata else {
        return;
    };

    for key in keys {
        let Some(value) = tool_metadata.get(*key) else {
            continue;
        };

        match value {
            Value::String(path) => files.push(path.clone()),
            Value::Array(paths) => {
                for path in paths.iter().filter_map(Value::as_str) {
                    files.push(path.to_string());
                }
            }
            _ => {}
        }
    }
}

pub(super) fn string_field<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
}
