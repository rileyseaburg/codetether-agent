//! Cursor movement helper.

use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Move the cursor to physical screen coordinates.
pub fn move_cursor(x: i32, y: i32) -> anyhow::Result<()> {
    unsafe { SetCursorPos(x, y) }?;
    Ok(())
}
