use serde_json::Value;
use std::collections::HashMap;

fn stub_marker_in_text(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "todo",
        "fixme",
        "placeholder implementation",
        "<placeholder>",
        "[placeholder]",
        "{placeholder}",
        "not implemented",
        "fallback",
        "stub",
        "coming soon",
        "unimplemented!(",
        "todo!(",
        "throw new error(\"not implemented",
    ];
    markers.into_iter().find(|m| lower.contains(m))
}

pub fn detect_stub_in_tool_input(tool_name: &str, tool_input: &Value) -> Option<String> {
    let check = |label: &str, text: &str| {
        stub_marker_in_text(text).map(|marker| format!("{label} contains stub marker \"{marker}\""))
    };

    match tool_name {
        "write" => tool_input
            .get("content")
            .and_then(Value::as_str)
            .and_then(|text| check("content", text)),
        "edit" | "confirm_edit" => tool_input
            .get("new_string")
            .or_else(|| tool_input.get("newString"))
            .and_then(Value::as_str)
            .and_then(|text| check("new_string", text)),
        "advanced_edit" => tool_input
            .get("newString")
            .and_then(Value::as_str)
            .and_then(|text| check("newString", text)),
        "multiedit" | "confirm_multiedit" => {
            let edits = tool_input.get("edits").and_then(Value::as_array)?;
            for (idx, edit) in edits.iter().enumerate() {
                if let Some(reason) = edit
                    .get("new_string")
                    .or_else(|| edit.get("newString"))
                    .and_then(Value::as_str)
                    .and_then(|text| check(&format!("edits[{idx}].new_string"), text))
                {
                    return Some(reason);
                }
            }
            None
        }
        "patch" => {
            let patch = tool_input.get("patch").and_then(Value::as_str)?;
            let added_lines = patch
                .lines()
                .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
                .map(|line| line.trim_start_matches('+'))
                .collect::<Vec<_>>()
                .join("\n");
            if added_lines.is_empty() {
                None
            } else {
                check("patch additions", &added_lines)
            }
        }
        _ => None,
    }
}

pub fn normalize_tool_call_for_execution(tool_name: &str, tool_input: &Value) -> (String, Value) {
    let mut normalized_name = tool_name.to_string();
    let mut normalized_input = tool_input.clone();

    if tool_name == "advanced_edit" {
        normalized_name = "edit".to_string();
    }

    if matches!(
        normalized_name.as_str(),
        "edit" | "confirm_edit" | "advanced_edit"
    ) {
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

    if matches!(normalized_name.as_str(), "multiedit" | "confirm_multiedit")
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

    (normalized_name, normalized_input)
}

/// Build a confirmation-apply request from a pending edit/multiedit tool call.
/// Returns `Some((confirm_tool_name, confirm_input))` if the tool call can be
/// auto-confirmed, or `None` if it is not a confirmable edit tool.
pub fn build_pending_confirmation_apply_request(
    tool_name: &str,
    tool_input: &Value,
    _tool_metadata: Option<&HashMap<String, Value>>,
) -> Option<(String, Value)> {
    let confirm_tool_name = match tool_name {
        "edit" | "confirm_edit" | "advanced_edit" => "confirm_edit",
        "multiedit" | "confirm_multiedit" => "confirm_multiedit",
        _ => return None,
    };

    let (_, mut normalized) = normalize_tool_call_for_execution(tool_name, tool_input);
    if let Some(obj) = normalized.as_object_mut() {
        obj.insert("confirm".to_string(), Value::Bool(true));
    }

    Some((confirm_tool_name.to_string(), normalized))
}
