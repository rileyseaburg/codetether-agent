//! Shared byte-level snapshot storage with native persistence notifications.
#[path = "persistence/notify.rs"]
pub(super) mod notify;
pub(super) use codetether_snapshot_io::{forget, is_unchanged, record_saved};
