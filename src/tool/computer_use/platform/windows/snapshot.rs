//! Native screen capture via Win32 GDI.

use crate::platform::windows::computer_use::capture_screenshot;

/// Capture a screenshot using native Win32 GDI BitBlt.
///
/// Returns a base64-encoded PNG data URL suitable for LLM vision input.
pub async fn handle_snapshot(
    _input: &crate::tool::computer_use::input::ComputerUseInput,
) -> anyhow::Result<crate::tool::ToolResult> {
    let (png, width, height, vx, vy) = capture_screenshot()?;
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png);
    Ok(super::response::success_result(serde_json::json!({
        "captured": true,
        "mime_type": "image/png",
        "data_url": format!("data:image/png;base64,{b64}"),
        "width": width,
        "height": height,
        "left": vx,
        "top": vy
    })))
}
