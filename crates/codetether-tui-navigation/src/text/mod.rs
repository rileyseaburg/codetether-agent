//! Character/backspace routing across mentions, filters and chat input.
//!
//! [character] and [backspace] choose native [Action] operations through [Host],
//! preserving mention priority, filter status and chat input-mode transitions.
mod action;
mod backspace;
mod character;
mod host;
pub use action::Action;
pub use backspace::backspace;
pub use character::character;
pub use host::Host;
