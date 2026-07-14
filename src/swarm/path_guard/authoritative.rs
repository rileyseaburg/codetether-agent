//! Authoritative workspace defaults for tools with optional roots.

use serde_json::Value;
use std::path::Path;

pub(super) fn apply(tool: &str, args: &mut Value, root: &Path) {
    let Some(fields) = args.as_object_mut() else {
        return;
    };
    let root = Value::String(root.display().to_string());
    match tool {
        "bash" | "git" => {
            fields.insert("cwd".to_string(), root);
        }
        "grep" => {
            fields.entry("path".to_string()).or_insert(root);
        }
        _ => {}
    }
}
