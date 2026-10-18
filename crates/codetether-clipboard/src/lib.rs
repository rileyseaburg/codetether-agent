//! Shared system clipboard capture and text writes.
//!
//! [capture_image] reads a PNG attachment and [copy_text] writes clipboard text.
//! The platform implementations use arboard or the raw-dylib Windows [winapi]
//! backend. Image representation is owned by [codetether_image].
#[cfg(not(windows))]
mod platform;
pub mod winapi;
#[cfg(windows)]
mod windows;
#[cfg(not(windows))]
pub use platform::{capture_image, copy_text};
#[cfg(windows)]
pub use windows::{capture_image, copy_text};
