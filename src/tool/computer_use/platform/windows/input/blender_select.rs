//! Blender select-and-frame sequence.

use crate::platform::windows::computer_use::send_text;
use crate::tool::computer_use::input::ComputerUseInput;

use super::{blender_evidence, key::press_key_name};

const SEQUENCE: &[&str] = &[
    "focus",
    "F3",
    "Select Pattern",
    "ENTER",
    "object_name",
    "ENTER",
    "NUMPAD_DOT",
    "NUMPAD_DOT",
];

pub fn run(
    input: &ComputerUseInput,
    name: &str,
    target: (i32, i32),
) -> anyhow::Result<serde_json::Value> {
    crate::platform::windows::computer_use::send_click(target.0, target.1)?;
    press_key_name("ESC")?;
    press_key_name("F3")?;
    send_text("Select Pattern")?;
    pause();
    press_key_name("ENTER")?;
    pause();
    send_text(name)?;
    press_key_name("ENTER")?;
    press_key_name("NUMPAD_DOT")?;
    press_key_name("NUMPAD_DOT")?;
    Ok(serde_json::json!({
        "sequence": SEQUENCE,
        "visual_evidence": blender_evidence::after_action(input.hwnd),
        "verified_by_pixels": false,
        "verified_by_blender_state": false
    }))
}

fn pause() {
    std::thread::sleep(std::time::Duration::from_millis(150));
}
