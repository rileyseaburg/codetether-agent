//! Normalize single-edit aliases without overwriting canonical fields.
use serde_json::Value;
pub(super) fn apply(normalized_name: &str, normalized_input: &mut Value) {
    if matches!(normalized_name, "edit" | "confirm_edit" | "advanced_edit") {
        if let Some(obj) = normalized_input.as_object_mut() {
            if obj.get("path").is_none() {
                if let Some(v) = obj.get("filePath").cloned() {
                    obj.insert("path".to_string(), v);
                } else if let Some(v) = obj.get("file").cloned() {
                    obj.insert("path".to_string(), v);
                }
            }
            if obj.get("old_string").is_none()
                && let Some(v) = obj.get("oldString").cloned()
            {
                obj.insert("old_string".to_string(), v);
            }
            if obj.get("new_string").is_none()
                && let Some(v) = obj.get("newString").cloned()
            {
                obj.insert("new_string".to_string(), v);
            }
        }
    }
}
