//! Recorded bus ingress, peer tracking and safe inbox routing.
//!
//! [drain] observes retained envelopes once using the frontend cursor. [track]
//! updates peer presence; [maybe_queue] separates managed-child results from
//! admissible external work, retaining the untrusted-input wrapper.
mod agent_track;
mod host;
mod inbox;
mod ingest;
mod owned_result;
mod prompt;
pub use agent_track::track;
pub use host::{Effects, Host, State};
pub use inbox::maybe_queue;
pub use ingest::drain;
#[cfg(test)]
mod tests;
