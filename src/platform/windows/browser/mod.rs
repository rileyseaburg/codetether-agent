//! Windows-native browser helpers via Win32 + registry.
//!
//! Replaces `where.exe` subprocess calls and static path probing with:
//! - Registry-based Chrome/Edge discovery
//! - Process enumeration to detect running debuggable browsers
//! - Window–PID correlation for native input coordination

mod discover;
mod process;
mod query;
mod registry;

pub use discover::find_browser;
pub use process::find_debug_browser;
pub use registry::registry_browser_path;
