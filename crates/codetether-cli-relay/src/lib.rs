//! Protocol-first CLI relay runtime.
//!
//! [run_protocol_first_relay] owns planning, correlated handoffs, refinement,
//! dynamic spawning, progress recording and final reporting. [Host] supplies
//! native provider loading and sink setup; [Session] supplies coding sessions.
mod convergence;
mod execute;
mod host;
pub use execute::run_protocol_first_relay;
pub use host::{Host, Session};
