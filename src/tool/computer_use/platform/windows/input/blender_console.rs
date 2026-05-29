//! Blender Python console execution helpers.

use super::{blender_clipboard, blender_timing, key::press_key_name};

pub fn exec(target: (i32, i32), code: &str) -> anyhow::Result<()> {
    crate::platform::windows::computer_use::send_click(target.0, target.1)?;
    blender_timing::settle();
    press_key_name("SHIFT F4")?;
    blender_timing::dialog();
    blender_clipboard::paste(&format!("exec({code:?})"))?;
    blender_timing::settle();
    press_key_name("ENTER")?;
    blender_timing::dialog();
    Ok(())
}
