//! Blender select-frame result assembly.

use serde_json::{Value, json};

pub fn build(name: &str, details: Value) -> crate::tool::ToolResult {
    let confirmed = details["matched_requested_object"]
        .as_bool()
        .unwrap_or(false);
    let method = method(&details);
    let mut result = super::report::mouse_result(json!({
        "blender_select_frame": true,
        "ui_sequence_sent": details["ui_recovery_state"].is_object(),
        "scripted_sequence_sent": true,
        "selection_method": method,
        "object_name": name,
        "visual_evidence": details["visual_evidence"].clone(),
        "selected_object_names": details["selected_object_names"].clone(),
        "active_object_name": details["active_object_name"].clone(),
        "confirmed_selected": confirmed,
        "confirmed_framed": details["confirmed_framed"].clone(),
        "verified_by_blender_state": details["verified_by_blender_state"].clone(),
        "verification_required_for_success": true,
        "details": details,
        "note": note(confirmed, method.as_str())
    }));
    result.success = confirmed;
    result
}

fn method(details: &Value) -> String {
    details["scripted_selection_state"]["selection_method"]
        .as_str()
        .unwrap_or("ui_recovery")
        .to_string()
}

fn note(confirmed: bool, method: &str) -> String {
    if confirmed {
        return format!("Selection verified from Blender state via {method}.");
    }
    "Selection command was attempted, but requested object selection was not verified.".to_string()
}
