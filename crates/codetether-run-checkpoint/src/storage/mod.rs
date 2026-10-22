//! Checkpoint sidecar lifecycle with native session metadata callbacks.
//!
//! Implement [Storage] on the session. [save] commits canonical metadata before
//! atomically replacing the fallback sidecar; [load] prefers canonical metadata.
//! [clear] removes a completed run's checkpoint.
mod clear;
mod host;
mod load;
mod path;
mod replace;
mod save;
pub use clear::clear;
pub use host::Storage;
pub use load::load;
pub use save::save;
