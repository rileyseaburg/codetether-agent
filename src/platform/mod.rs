//! Platform abstraction layer.
//!
//! Provides platform-specific implementations for audio capture, playback,
//! and other OS-level features that differ between Windows, macOS, and Linux.

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "windows"))]
pub mod unix;
