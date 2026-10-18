//! On-disk persistence: save, load, delete, and directory lookup.

use super::types::Session;

codetether_session_store::impl_session_lifecycle_host!(
    Session;
    super::store::load,
    super::listing::list_sessions_for_directory,
    super::list_sessions,
    super::index::recall::schedule,
    super::index_produce::notify::saved,
    super::store::archive::schedule,
    super::index_produce::notify::removed,
    super::index::recall::remove
);
#[cfg(test)]
mod location;
#[cfg(test)]
#[path = "save_guard.rs"]
mod save_guard;
#[cfg(test)]
#[path = "persistence_snapshot.rs"]
mod snapshot;

codetether_session_schema::impl_session_persistence!(Session);
