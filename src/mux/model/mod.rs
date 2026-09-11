//! In-memory mux server, session, and window state.
//!
//! One [`MuxSnapshot`] describes one server process bound to one checkout.
//! It hosts many [`MuxSession`]s, each an isolated runtime with its own
//! windows, active window, and TUI status.

mod actions;
mod legacy;
mod runtime;
mod server;
mod session;
mod sessions;
mod window;

pub(crate) use runtime::MuxRuntimeStatus;
pub(super) use server::MuxSnapshot;
pub(super) use session::MuxSession;
pub(super) use window::MuxWindow;
