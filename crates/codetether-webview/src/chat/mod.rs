//! Transcript scrolling, composer presentation and minimum terminal size.
//! [render_center] and [render_input] draw already-prepared content.
mod center;
mod input;
mod size;
pub use center::render_center;
pub use input::render_input;
pub use size::terminal_too_small;
