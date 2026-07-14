//! Focused helpers for session metadata serialization and debug output.

#[path = "metadata_debug.rs"]
mod debug;
#[path = "metadata_debug_sink.rs"]
mod sink;

pub(super) fn slash_autocomplete() -> bool {
    true
}

pub(super) fn use_worktree() -> bool {
    true
}
