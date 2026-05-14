//! Raw device input handlers for native browser pages.

mod keyboard;
mod mouse;
mod page;

/// Keyboard input handlers.
pub(super) use keyboard::{keyboard_press, keyboard_type};
/// Mouse input handler.
pub(super) use mouse::mouse_click;
