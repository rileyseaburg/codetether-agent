//! Normalize multi-edit aliases without overwriting canonical fields.
use serde_json::Value;
pub(super) fn apply(normalized_name: &str, normalized_input: &mut Value) {
    if matches!(normalized_name, "multiedit" | "confirm_multiedit")
        && let Some(obj) = normalized_input.as_object_mut()
    {
        if obj.get("edits").is_none() {
            if let Some(v) = obj.get("changes").cloned() {
                obj.insert("edits".to_string(), v);
            } else if let Some(v) = obj.get("operations").cloned() {
                obj.insert("edits".to_string(), v);
            } else if obj.get("file").is_some() || obj.get("path").is_some() {
                let single = Value::Object(obj.clone());
                obj.insert("edits".to_string(), Value::Array(vec![single]));
            }
        }

        if let Some(edits) = obj.get_mut("edits").and_then(Value::as_array_mut) {
            for edit in edits {
                if let Some(edit_obj) = edit.as_object_mut() {
                    if edit_obj.get("file").is_none() {
                        if let Some(v) = edit_obj.get("filePath").cloned() {
                            edit_obj.insert("file".to_string(), v);
                        } else if let Some(v) = edit_obj.get("path").cloned() {
                            edit_obj.insert("file".to_string(), v);
                        }
                    }
                    if edit_obj.get("old_string").is_none()
                        && let Some(v) = edit_obj.get("oldString").cloned()
                    {
                        edit_obj.insert("old_string".to_string(), v);
                    }
                    if edit_obj.get("new_string").is_none()
                        && let Some(v) = edit_obj.get("newString").cloned()
                    {
                        edit_obj.insert("new_string".to_string(), v);
                    }
                }
            }
        }
    }
}
