//! Hand-off of inbound A2A turns to the interactive TUI session.
//!
//! Without this, `message/send` from a LAN peer runs in a headless session
//! that shares nothing with what the human sees. When a TUI is attached
//! and has opted in, the server instead parks the turn here; the TUI
//! dequeues it as a prompt, runs it in the live session, and posts the
//! assistant's final text back through the matching [`Pending`] slot.
//!
//! The server blocks on the slot so the peer still receives a normal
//! `Completed` task in the HTTP response, exactly as with headless runs.

mod pending;
mod queue;
mod registry;

#[cfg(test)]
mod tests;

pub use pending::{Outcome, Pending, Responder};
pub use queue::{Inbound, dequeue, enqueue};
pub use registry::{attach, detach, is_attached};
