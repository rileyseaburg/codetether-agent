//! Exact-session restart primitives using existing mux lifecycle operations.
//!
//! [wait_stopped] waits for registry disappearance. [start_exact] recreates
//! a named session in the supplied workspace, joining a live workspace server
//! or launching a fresh server as necessary.
mod start;
mod wait;
pub use start::start_exact;
pub use wait::wait_stopped;
