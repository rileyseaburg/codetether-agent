//! Linux desktop session detection and bounded screenshot execution.
//!
//! [dispatch] supports status and whole-desktop snapshots using installed,
//! session-compatible capture utilities. Capture subprocesses retain their
//! existing timeout and process-tree ownership; this backend never injects input.
#![cfg(target_os = "linux")]
mod linux;
pub use linux::dispatch;
