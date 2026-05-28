//! Blender select-frame result assembly.

use serde_json::{Value, json};

pub fn build(name: &str, details: Value) -> crate::tool::ToolResult {
    let confirmed = details["matched_requested_object"]
        .as_bool()
        .unwrap_or(false);
    let mut result = super::report::mouse_result(json!({
        "blender_select_frame": true,
        "ui_sequence_sent": true,
        "object_name": name,
        "visual_evidence": details["visual_evidence"].clone(),
        "selected_object_names": details["selected_object_names"].clone(),
        "active_object_name": details["active_object_name"].clone(),
        "confirmed_selected": confirmed,
        "confirmed_framed": details["confirmed_framed"].clone(),
        "verified_by_blender_state": details["verified_by_blender_state"].clone(),
        "verification_required_for_success": true,
        "details": details,
        "note": note(confirmed)
    }));
    result.success = confirmed;
    result
}

fn note(confirmed: bool) -> &'static str {
    if confirmed {
        "Selection verified from Blender state; frame command was sent via F3."
    } else {
        "UI sequence sent, but requested object selection was not verified."
    }
}
