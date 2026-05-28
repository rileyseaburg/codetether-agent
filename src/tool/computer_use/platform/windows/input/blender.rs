//! Blender viewport helper actions.

use crate::platform::windows::computer_use::{send_text, window::bring_to_front};
use crate::tool::computer_use::input::ComputerUseInput;

use super::{key::press_key_name, validate};

pub async fn handle_focus_viewport(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let target = focus_target(input)?;
    super::modifiers::with_modifiers(&input.modifiers, || {
        crate::platform::windows::computer_use::send_click(target.0, target.1)
    })?;
    press_key_name("ESC")?;
    Ok(super::report::mouse_result(serde_json::json!({
        "focused_viewport": true,
        "target": {"x": target.0, "y": target.1},
        "coordinate_mode": validate::coordinate_mode(input)
    })))
}

pub async fn handle_blender_select_frame(
    input: &ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let name = input
        .object_name
        .as_deref()
        .or(input.text.as_deref())
        .ok_or_else(|| anyhow::anyhow!("object_name or text is required"))?;
    let target = focus_target(input)?;
    crate::platform::windows::computer_use::send_click(target.0, target.1)?;
    press_key_name("ESC")?;
    press_key_name("F3")?;
    send_text("Select Pattern")?;
    std::thread::sleep(std::time::Duration::from_millis(150));
    press_key_name("ENTER")?;
    std::thread::sleep(std::time::Duration::from_millis(150));
    send_text(name)?;
    press_key_name("ENTER")?;
    press_key_name("NUMPAD_DOT")?;
    Ok(super::report::mouse_result(serde_json::json!({
        "blender_select_frame": true,
        "object_name": name,
        "target": {"x": target.0, "y": target.1},
        "sequence": ["focus", "F3", "Select Pattern", "ENTER", "object_name", "ENTER", "NUMPAD_DOT"],
        "note": "Uses Blender UI search; select failures still require inspecting the next window_snapshot."
    })))
}

fn focus_target(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    if let Some(hwnd) = input.hwnd {
        let _ = bring_to_front(hwnd)?;
    }
    validate::coords(input)
}
