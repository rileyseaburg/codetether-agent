//! Blender frame-selected command sequence.

use codetether_platform::windows::computer_use::send_text;

use super::{blender_timing, key::press_key_name};

pub fn run(target: (i32, i32)) -> anyhow::Result<()> {
    codetether_platform::windows::computer_use::send_click(target.0, target.1)?;
    blender_timing::settle();
    press_key_name("F3")?;
    blender_timing::dialog();
    send_text("Frame Selected")?;
    blender_timing::dialog();
    press_key_name("ENTER")?;
    blender_timing::dialog();
    Ok(())
}
