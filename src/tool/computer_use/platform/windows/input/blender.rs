//! Blender viewport helper actions.

use crate::platform::windows::computer_use::window::bring_to_front;
use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_focus_viewport(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let target = focus_target(input)?;
    super::modifiers::with_modifiers(&input.modifiers, || {
        crate::platform::windows::computer_use::send_click(target.0, target.1)
    })?;
    super::key::press_key_name("ESC")?;
    Ok(super::report::mouse_result(serde_json::json!({
        "focused_viewport": true,
        "target": {"x": target.0, "y": target.1},
        "coordinate_mode": super::validate::coordinate_mode(input)
    })))
}

pub async fn handle_blender_select_frame(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let name = name(input)?;
    let target = focus_target(input)?;
    let details = super::blender_select::run(input, name, target)?;
    Ok(super::report::mouse_result(serde_json::json!({
        "blender_select_frame": true,
        "object_name": name,
        "target": {"x": target.0, "y": target.1},
        "visual_evidence": details["visual_evidence"].clone(),
        "details": details,
        "confirmed_selected": false,
        "note": "UI sequence sent only; inspect visual_evidence before editing."
    })))
}

fn focus_target(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    if let Some(hwnd) = input.hwnd {
        let _ = bring_to_front(hwnd)?;
    }
    super::validate::coords(input)
}

fn name(input: &ComputerUseInput) -> anyhow::Result<&str> {
    input
        .object_name
        .as_deref()
        .or(input.text.as_deref())
        .ok_or_else(|| anyhow::anyhow!("object_name or text is required"))
}
