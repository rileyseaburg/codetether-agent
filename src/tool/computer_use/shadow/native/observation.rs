//! Read-only hardware observations, not claims of immutable desktop state.
use serde_json::{Value, json};
use windows::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::{GetCursorPos, GetForegroundWindow},
};

pub(in super::super) fn observe() -> Value {
    let mut cursor = POINT::default();
    let cursor = match unsafe { GetCursorPos(&mut cursor) } {
        Ok(()) => json!({ "x": cursor.x, "y": cursor.y }),
        Err(error) => json!({ "error": error.to_string() }),
    };
    let foreground = unsafe { GetForegroundWindow() }.0 as isize as i64;
    json!({
        "cursor": cursor, "foreground_hwnd": foreground,
        "interpretation": "Read-only sample; user or application activity can change cursor/foreground independently"
    })
}
