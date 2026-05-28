//! Blender select-and-frame sequence.

use crate::tool::computer_use::input::ComputerUseInput;
use serde_json::json;

use super::{blender_evidence, blender_pattern, blender_query};

pub fn run(
    input: &ComputerUseInput,
    name: &str,
    target: (i32, i32),
    focused_hwnd: i64,
) -> anyhow::Result<serde_json::Value> {
    let pattern = blender_pattern::select_pattern(name);
    super::blender_select_ui::run(target, &pattern)?;
    let selected = blender_query::selected_names(target);
    super::blender_select_ui::click(target)?;
    super::blender_frame::run(target)?;
    let after = blender_query::selected_names(target);
    let names = blender_query::names_vec(&after);
    Ok(json!({
        "sequence": super::blender_sequence::SELECT_FRAME,
        "select_pattern": pattern,
        "focused_hwnd": focused_hwnd,
        "viewport_child_hwnd": input.viewport_child_hwnd,
        "client_area": input.client_area,
        "target": {"x": target.0, "y": target.1},
        "selection_state": selected,
        "selection_state_after_frame": after,
        "selected_object_names": names,
        "active_object_name": after["active_object_name"].clone(),
        "matched_requested_object": blender_pattern::matched(name, &names),
        "confirmed_framed": serde_json::Value::Null,
        "visual_evidence": blender_evidence::after_action(input.hwnd),
        "verified_by_pixels": false,
        "verified_by_blender_state": after["available"].as_bool().unwrap_or(false)
    }))
}
