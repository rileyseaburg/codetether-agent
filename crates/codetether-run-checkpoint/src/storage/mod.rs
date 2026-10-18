//! Checkpoint sidecar lifecycle with native session metadata callbacks.
//!
//! Implement [Storage] on the session; [save], [load] and [clear] retain the
//! original sidecar operation order and native session save behavior.
mod clear;
mod host;
mod load;
mod path;
mod save;
pub use clear::clear;
pub use host::Storage;
pub use load::load;
pub use save::save;
