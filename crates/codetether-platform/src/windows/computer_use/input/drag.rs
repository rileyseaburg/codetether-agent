//! Click-drag via Win32 SendInput.

use super::{
    drag_button, drag_motion::drag_motion, hold_modifiers, modifier_vks, move_cursor,
    release_modifiers, send_mouse_button,
};

/// Move cursor to (x1, y1), depress a button, drag to (x2, y2), release.
pub fn send_drag(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    button: Option<&str>,
    modifiers: &[String],
    steps: Option<u32>,
    duration_ms: Option<u64>,
) -> anyhow::Result<&'static str> {
    let button = drag_button(button)?;
    let modifiers = modifier_vks(modifiers)?;
    unsafe { drag_inner((x1, y1), (x2, y2), &button, &modifiers, steps, duration_ms) }?;
    Ok(button.name)
}

unsafe fn drag_inner(
    start: (i32, i32),
    end: (i32, i32),
    button: &super::mouse_button::MouseButtonFlags,
    modifiers: &[u16],
    steps: Option<u32>,
    duration_ms: Option<u64>,
) -> anyhow::Result<()> {
    move_cursor(start.0, start.1)?;
    hold_modifiers(modifiers)?;
    send_mouse_button(button.down, "down")?;
    drag_motion(start, end, steps, duration_ms)?;
    send_mouse_button(button.up, "up")?;
    release_modifiers(modifiers)
}
