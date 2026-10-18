//! Play-break activation and interactive redraw policy.
//!
//! [handle_event] consumes runtime play-break events. [handle] forwards keys
//! only while a game is active and requests redraw only for handled input.
mod event;
mod key;
pub use event::handle_event;
pub use key::handle;
