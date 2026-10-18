//! Local filesystem store for the TUI model picker list.
//!
//! See [`ModelStore`] for the entry point. The store keeps the last
//! successfully fetched model identifiers on disk so the picker can render
//! instantly while a fresh list loads in the background.

mod persist;
mod store;

pub use store::ModelStore;

#[cfg(test)]
mod tests;
