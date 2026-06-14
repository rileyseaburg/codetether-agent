//! Native Win32 computer-use operations.
//!
//! Replaces PowerShell-mediated operations with direct Win32 API calls:
//! - Screen capture via GDI `BitBlt`
//! - Input injection via `SendInput`
//! - Window/process enumeration via `EnumWindows` / `ToolHelp32`

mod cursor;
pub mod encode;
pub mod input;
pub mod process;
mod screen_gdi;
mod screen_metrics;
pub mod snapshot;
pub mod window;
pub mod windows;

pub use cursor::cursor_position;
pub use input::{
    hold_modifiers, modifier_vks, mouse_down, mouse_up, move_cursor, parse_send_keys,
    release_modifiers, send_chord, send_click, send_double_click, send_drag, send_key,
    send_right_click, send_scroll, send_text,
};
pub use process::list_processes;
pub use snapshot::capture_screenshot;
pub use window::{
    bring_to_front, capture_window_png, client_point_to_screen, set_foreground_window_text,
};
pub use windows::list_windows;
