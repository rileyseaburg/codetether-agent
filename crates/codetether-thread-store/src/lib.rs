//! # Thread Event Store
//!
//! Persists thread and turn events as append-only JSON Lines files. A
//! [`ThreadStore`] points at a directory, and each thread is stored in a
//! traversal-safe `<thread-id>.jsonl` file.

mod append;
mod path;
mod read;
mod store;
mod types;

pub use store::ThreadStore;
pub use types::ThreadEvent;

#[cfg(test)]
mod tests;
