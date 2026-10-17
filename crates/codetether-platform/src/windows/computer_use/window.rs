//! Window focus and capture via Win32.

pub mod active;
pub mod bounds;
pub mod capture;
pub mod client;
pub mod client_to_screen;
pub mod focus;
pub mod set_text;

pub use active::foreground_window;
pub use bounds::window_bounds;
pub use capture::capture_window_png;
pub use client::client_origin;
pub use client_to_screen::client_point_to_screen;
pub use focus::bring_to_front;
pub use set_text::set_foreground_window_text;
