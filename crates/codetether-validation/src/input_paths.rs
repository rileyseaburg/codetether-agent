//! Extract tool-specific edit paths in their original precedence order.
use super::metadata::{collect_metadata_paths, string_field};
use serde_json::Value;
use std::collections::HashMap;
pub(super) fn collect(
    tool_name: &str,
    tool_input: &Value,
    tool_metadata: Option<&HashMap<String, Value>>,
    files: &mut Vec<String>,
) {
    match tool_name {
        "write" | "edit" | "confirm_edit" => {
            if let Some(path) = string_field(tool_input, &["path"]) {
                files.push(path.to_string());
            }
        }
        "advanced_edit" => {
            if let Some(path) = string_field(tool_input, &["filePath", "file_path", "path"]) {
                files.push(path.to_string());
            }
        }
        "multiedit" | "confirm_multiedit" => {
            if let Some(edits) = tool_input.get("edits").and_then(Value::as_array) {
                for edit in edits {
                    if let Some(path) = string_field(edit, &["path", "filePath", "file_path"]) {
                        files.push(path.to_string());
                    }
                }
            }
        }
        "patch" => {
            collect_metadata_paths(files, tool_metadata, &["files"]);
        }
        _ => {}
    }
}
