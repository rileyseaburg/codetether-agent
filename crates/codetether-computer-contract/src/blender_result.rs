//! Blender select-frame result assembly.

use serde_json::{Value, json};

/// Assemble a Blender selection report without embedding image data in its text.
/// # Arguments
/// Receives the object name, verification details and native metadata formatter.
/// # Returns
/// A tool result whose success follows verified selection; image data is metadata only.
/// # Examples
/// ```
/// let result = codetether_computer_contract::blender_result::build(
///     "Cube", serde_json::json!({"matched_requested_object": true}),
///     codetether_computer_contract::response::success_result,
/// );
/// assert!(result.success);
/// ```
pub fn build(
    name: &str,
    mut details: Value,
    report: impl FnOnce(Value) -> codetether_tool_core::ToolResult,
) -> codetether_tool_core::ToolResult {
    // Remove before either evidence copy is serialized into tool text.
    let image = details
        .get_mut("visual_evidence")
        .and_then(Value::as_object_mut)
        .and_then(|evidence| evidence.remove("image_data_url"));
    let confirmed = details["matched_requested_object"]
        .as_bool()
        .unwrap_or(false);
    let method = method(&details);
    let mut result = report(json!({
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
    if let Some(image) = image {
        result = result.with_metadata("image_data_url", image);
    }
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

#[cfg(test)]
#[path = "blender_result_tests.rs"]
mod tests;
