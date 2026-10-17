//! Stateful mouse button primitives.

use super::{drag_button, send_mouse_button};

pub fn mouse_down(button: Option<&str>) -> anyhow::Result<&'static str> {
    let button = drag_button(button)?;
    send_mouse_button(button.down, "down")?;
    Ok(button.name)
}

pub fn mouse_up(button: Option<&str>) -> anyhow::Result<&'static str> {
    let button = drag_button(button)?;
    send_mouse_button(button.up, "up")?;
    Ok(button.name)
}
