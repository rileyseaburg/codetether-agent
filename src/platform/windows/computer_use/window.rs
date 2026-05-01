//! Window focus and capture via Win32.

pub mod capture;
pub mod focus;

pub use capture::capture_window_jpeg;
pub use focus::bring_to_front;
