//! Blender viewport focus helpers.

use crate::tool::computer_use::input::ComputerUseInput;

pub fn focus_child(input: &ComputerUseInput) -> anyhow::Result<i64> {
    let hwnd = input
        .viewport_child_hwnd
        .or(input.hwnd)
        .ok_or_else(|| anyhow::anyhow!("hwnd is required"))?;
    let _ = crate::platform::windows::computer_use::window::bring_to_front(hwnd)?;
    Ok(hwnd)
}

pub fn target(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    if input.x.is_some() || input.y.is_some() {
        return super::validate::coords(input);
    }
    let hwnd = input.viewport_child_hwnd.or(input.hwnd);
    let hwnd = hwnd.ok_or_else(|| anyhow::anyhow!("hwnd is required"))?;
    center(hwnd, input.client_area)
}

fn center(hwnd: i64, client_area: bool) -> anyhow::Result<(i32, i32)> {
    let bounds = crate::platform::windows::computer_use::window::window_bounds(hwnd)?;
    let (left, top) = if client_area {
        crate::platform::windows::computer_use::window::client_origin(hwnd)?
    } else {
        (bounds.left, bounds.top)
    };
    let width = bounds.right - bounds.left;
    let height = bounds.bottom - bounds.top;
    Ok((left + width / 2, top + height / 2))
}
