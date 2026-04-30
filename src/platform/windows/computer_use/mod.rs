//! Native Win32 computer-use operations.
//!
//! Replaces PowerShell-mediated operations with direct Win32 API calls:
//! - Screen capture via GDI `BitBlt`
//! - Input injection via `SendInput`
//! - Window/process enumeration via `EnumWindows` / `ToolHelp32`

pub mod encode;
pub mod input;
pub mod process;
pub mod snapshot;
pub mod windows;

pub use snapshot::capture_screenshot;
pub use input::{send_click, send_key, send_scroll, send_text};
pub use windows::list_windows;
pub use process::list_processes;
