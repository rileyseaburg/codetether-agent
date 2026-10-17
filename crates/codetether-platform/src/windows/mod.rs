//! Windows platform integrations.
//!
//! Uses the `windows` crate for native Windows APIs:
//! - WASAPI audio capture/playback for low-latency voice
//! - GDI screen capture, SendInput, window enumeration
//! - Process enumeration via ToolHelp32
//! - Registry-based browser discovery for chromiumoxide

pub mod browser;
pub mod computer_use;
pub mod voice;
