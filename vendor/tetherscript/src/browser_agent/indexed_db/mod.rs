//! Deterministic IndexedDB-like storage for browser-agent contexts.
//!
//! The model is deliberately native Rust data. It stores string records by
//! origin, database name, object-store name, and key without performing I/O.

mod access;
mod context_api;
mod context_read;
mod delete;
mod list;
mod origin;
mod page_api;
mod page_read;
mod page_state;
mod record;
mod replace;
mod store;

#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tests;

pub(crate) use origin::indexed_db_origin;
pub use record::IndexedDbRecord;
pub use store::IndexedDbStore;
