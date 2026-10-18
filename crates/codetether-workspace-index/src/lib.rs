//! Best-effort workspace-to-session index for fast conversation resume.
//!
//! [WorkspaceIndex] holds canonical workspace keys and session IDs. [Source]
//! resolves storage without coupling this crate to application sessions.
//! Use [load_sync] to read the cache and [upsert_sync] to persist a mapping.
mod index;
mod lookup;
mod read;
mod source;
mod write;
pub use index::WorkspaceIndex;
pub use read::load_sync;
pub use source::Source;
pub use write::{save_sync, upsert_sync};
/// Sidecar filename excluded from normal session discovery.
pub const INDEX_FILENAME: &str = ".workspace_index.json";
