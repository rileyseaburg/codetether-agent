//! Mouse button mapping for native input.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct MouseButtonFlags {
    pub name: &'static str,
    pub down: MOUSE_EVENT_FLAGS,
    pub up: MOUSE_EVENT_FLAGS,
}

pub fn drag_button(name: Option<&str>) -> anyhow::Result<MouseButtonFlags> {
    match name.unwrap_or("left").to_ascii_lowercase().as_str() {
        "left" => Ok(flags("left", MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP)),
        "middle" => Ok(flags(
            "middle",
            MOUSEEVENTF_MIDDLEDOWN,
            MOUSEEVENTF_MIDDLEUP,
        )),
        "right" => Ok(flags("right", MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP)),
        other => anyhow::bail!("unsupported drag button: {other}"),
    }
}

fn flags(name: &'static str, down: MOUSE_EVENT_FLAGS, up: MOUSE_EVENT_FLAGS) -> MouseButtonFlags {
    MouseButtonFlags { name, down, up }
}
