//! Serializable mux server, session and window state with legacy decoding.
//!
//! [MuxSnapshot] models a single checkout hosting isolated [MuxSession] values.
//! Each session owns [MuxWindow] values and optional [MuxRuntimeStatus].
//! [isolation::Isolation] selects allocation policy without allocating worktrees.
mod actions;
pub mod isolation;
mod legacy;
mod runtime;
mod server;
mod session;
mod sessions;
mod window;
pub use runtime::MuxRuntimeStatus;
pub use server::MuxSnapshot;
pub use session::MuxSession;
pub use window::MuxWindow;
#[cfg(test)]
mod tests;
