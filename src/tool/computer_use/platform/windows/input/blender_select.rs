//! Blender select-and-frame sequence.

use crate::tool::computer_use::input::ComputerUseInput;
use serde_json::{Value, json};

use super::{
    blender_evidence, blender_pattern, blender_query, blender_select_meta, blender_select_python,
};

pub fn run(
    input: &ComputerUseInput,
    name: &str,
    target: (i32, i32),
    hwnd: i64,
) -> anyhow::Result<Value> {
    let scripted = blender_select_python::run(target, name);
    let ui_recovery = ui_recovery_if_needed(target, name, &scripted)?;
    let final_state = blender_select_meta::final_state(&scripted, &ui_recovery);
    let names = blender_query::names_vec(final_state);
    let active = final_state["active_object_name"].clone();
    let framed = final_state["framed"].clone();
    let matched = blender_pattern::matched(name, &names);
    let verified = blender_select_meta::available(final_state);
    let mut details = blender_select_meta::build(input, hwnd, target, scripted, ui_recovery);
    details["selected_object_names"] = json!(names);
    details["active_object_name"] = active;
    details["matched_requested_object"] = json!(matched);
    details["confirmed_framed"] = framed;
    details["visual_evidence"] = blender_evidence::after_action(input.hwnd);
    details["verified_by_pixels"] = json!(false);
    details["verified_by_blender_state"] = json!(verified);
    Ok(details)
}

fn ui_recovery_if_needed(
    target: (i32, i32),
    name: &str,
    scripted: &Value,
) -> anyhow::Result<Value> {
    if blender_select_meta::available(scripted) && !blender_query::names_vec(scripted).is_empty() {
        return Ok(Value::Null);
    }
    super::blender_select_ui::run(target, &blender_pattern::select_pattern(name))?;
    super::blender_frame::run(target)?;
    Ok(blender_query::selected_names(target))
}
