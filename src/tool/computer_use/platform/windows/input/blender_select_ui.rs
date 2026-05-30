//! Blender Select Pattern UI command sequence.

use crate::platform::windows::computer_use::send_text;

use super::{blender_timing, key::press_key_name};

pub fn run(target: (i32, i32), pattern: &str) -> anyhow::Result<()> {
    click(target)?;
    press_key_name("ESC")?;
    blender_timing::dialog();
    open_search("Select Pattern")?;
    send_text(pattern)?;
    blender_timing::dialog();
    press_key_name("ENTER")?;
    blender_timing::dialog();
    Ok(())
}

pub fn click(target: (i32, i32)) -> anyhow::Result<()> {
    crate::platform::windows::computer_use::send_click(target.0, target.1)?;
    blender_timing::dialog();
    Ok(())
}

fn open_search(command: &str) -> anyhow::Result<()> {
    press_key_name("F3")?;
    blender_timing::dialog();
    send_text(command)?;
    blender_timing::dialog();
    press_key_name("ENTER")?;
    blender_timing::dialog();
    Ok(())
}
