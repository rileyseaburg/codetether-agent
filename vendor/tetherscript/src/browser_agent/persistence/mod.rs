//! Native browser-agent state snapshots.
//!
//! This module keeps export/restore data as Rust structs instead of a JSON
//! format. Snapshots intentionally omit the live JavaScript heap; restoring a
//! page starts a fresh runtime over the restored DOM and storage state.

#[path = "context.rs"]
mod context;
#[path = "page.rs"]
mod page;
#[path = "page_restore.rs"]
mod page_restore;
#[path = "page_shared.rs"]
mod page_shared;
#[cfg(test)]
#[path = "../persistence_heap_tests.rs"]
mod persistence_heap_tests;
#[cfg(test)]
#[path = "../persistence_tests.rs"]
mod persistence_tests;
#[path = "storage.rs"]
mod storage;
#[path = "types.rs"]
mod types;

pub(crate) use storage::{restore_storage, snapshot_storage};
pub use types::{
    BrowserContextSnapshot, BrowserPageSnapshot, BrowserStorageState, StorageOriginSnapshot,
};
