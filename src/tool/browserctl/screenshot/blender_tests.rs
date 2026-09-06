//! Exercise real Blender result assembly on any OS with only cursor lookup mocked.

#[path = "../../computer_use/platform/windows/input/blender_select_result.rs"]
mod blender_select_result;
use serde_json::{Value, json};

mod report {
    pub fn mouse_result(output: serde_json::Value) -> crate::tool::ToolResult {
        crate::tool::ToolResult::success(serde_json::to_string(&output).unwrap())
    }
}

#[test]
fn blender_image_is_metadata_only_even_when_selection_is_unconfirmed() {
    let image = crate::tool::result_images::encoded(b"captured bytes", "image/png");
    let evidence = json!({"captured": true, "path": "capture.png", "width": 80,
        "height": 60, "review_required": true, "selection_overlay_detected": null});
    for confirmed in [true, false] {
        let mut details = json!({"visual_evidence": evidence,
            "matched_requested_object": confirmed, "confirmed_framed": true,
            "hwnd": 42, "target": [10, 20]});
        details["visual_evidence"]["image_data_url"] = image.clone();
        let result = blender_select_result::build("Cube", details);
        assert_eq!(result.success, confirmed);
        assert_eq!(result.metadata["image_data_url"], image);
        let output: Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(output["visual_evidence"], evidence);
        assert_eq!(output["details"]["visual_evidence"], evidence);
        assert_eq!(output["details"]["hwnd"], 42);
        assert_eq!(output["details"]["target"], json!([10, 20]));
        assert_eq!(output["confirmed_framed"], true);
        assert!(!result.output.contains("base64"));
        assert!(!result.output.contains("image_data_url"));
    }
}

#[test]
fn blender_capture_failure_retains_evidence_without_image_metadata() {
    let evidence = json!({"captured": false, "review_required": true, "path": null,
        "note": "No hwnd capture available; call window_snapshot next."});
    for evidence in [evidence, Value::Null] {
        let result = blender_select_result::build(
            "Cube",
            json!({
                "visual_evidence": evidence, "matched_requested_object": false
            }),
        );
        assert!(!result.success);
        assert!(!result.metadata.contains_key("image_data_url"));
        let output: Value = serde_json::from_str(&result.output).unwrap();
        assert_eq!(output["visual_evidence"], evidence);
    }
}
