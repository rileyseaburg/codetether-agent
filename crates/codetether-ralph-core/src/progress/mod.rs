//! Progress-file loading and formatted iteration appends.
//!
//! Callers provide workspace and configured path; reads retain the original
//! empty fallback and writes propagate filesystem errors.
mod append_progress;
mod load_progress;
pub use append_progress::append_progress;
pub use load_progress::load_progress;

mod completed_entry;
mod failed_entry;
mod parallel_entry;
pub use completed_entry::completed_entry;
pub use failed_entry::failed_entry;
pub use parallel_entry::parallel_entry;
