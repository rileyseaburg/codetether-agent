//! In-memory mux session and window state.

mod actions;
mod session;
mod window;

pub(super) use session::MuxSnapshot;
pub(super) use window::MuxWindow;
