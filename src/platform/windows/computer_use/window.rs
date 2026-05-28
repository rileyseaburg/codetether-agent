//! Window focus and capture via Win32.

pub mod active;
pub mod bounds;
pub mod capture;
pub mod focus;

pub use active::foreground_window;
pub use bounds::window_bounds;
pub use capture::capture_window_png;
pub use focus::bring_to_front;
