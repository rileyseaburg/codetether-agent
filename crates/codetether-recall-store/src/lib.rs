//! Recall persistence with native paths and projection reads supplied by a host.
//!
//! [upsert] and [remove] serialize sidecar/catalog mutations through [acquire].
//! [read] prefers a materialized projection; sidecar fallback checks its schema.
//! Deletion guards prevent late background writes from resurrecting indexes.
//! [RecallCatalog] maps one canonical workspace to its indexed sessions.
pub mod atomic;
mod catalog;
mod catalog_io;
mod host;
pub mod paths;
mod session_io;
mod store;
mod store_freshness;
mod store_lock;
pub mod tombstone;
pub use catalog::RecallCatalog;
pub use catalog_io::{read as read_catalog, write as write_catalog};
pub use host::Host;
pub use session_io::{read, remove as remove_sidecar, write};
pub use store::{remove, upsert};
pub use store_lock::acquire;
#[cfg(test)]
mod tests;

/// Implementation support for application-local session bindings.
#[doc(hidden)]
pub mod session_binding;
