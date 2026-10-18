//! Append-only borrowed access to a provider-message transcript.
//!
//! [History] exposes append and immutable reads while holding an exclusive
//! borrow of the underlying vector. [History::with_pages] also maintains
//! the parallel page classifications without exposing destructive mutation.
mod append;
mod history;
mod view;
pub use history::History;
#[cfg(test)]
mod tests;
