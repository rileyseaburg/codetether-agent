//! On-disk persistence: save, load, delete, and directory lookup.

use super::types::Session;

mod lifecycle_host;
#[cfg(test)]
mod location;
#[cfg(test)]
#[path = "save_guard.rs"]
mod save_guard;
#[cfg(test)]
#[path = "persistence_snapshot.rs"]
mod snapshot;
#[path = "workspace_resolve.rs"]
mod workspace_resolve;

codetether_session_schema::impl_session_persistence!(Session);
