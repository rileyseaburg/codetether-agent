//! Blender viewport helper actions.

use crate::tool::computer_use::input::ComputerUseInput;

pub async fn handle_focus_viewport(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let hwnd = super::blender_focus::focus_child(input)?;
    let target = super::blender_focus::target(input)?;
    super::modifiers::with_modifiers(&input.modifiers, || {
        crate::platform::windows::computer_use::send_click(target.0, target.1)
    })?;
    super::key::press_key_name("ESC")?;
    Ok(super::report::mouse_result(serde_json::json!({
        "focused_viewport": true,
        "hwnd": hwnd,
        "target": {"x": target.0, "y": target.1},
        "coordinate_mode": super::validate::coordinate_mode(input)
    })))
}

pub async fn handle_blender_select_frame(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let name = name(input)?;
    let focused_hwnd = super::blender_focus::focus_child(input)?;
    let target = super::blender_focus::target(input)?;
    let details = super::blender_select::run(input, name, target, focused_hwnd)?;
    Ok(super::blender_select_result::build(name, details))
}

fn name(input: &ComputerUseInput) -> anyhow::Result<&str> {
    input
        .object_name
        .as_deref()
        .or(input.text.as_deref())
        .ok_or_else(|| anyhow::anyhow!("object_name or text is required"))
}
