//! Shared worktree-independent location pointers with native journal regressions.
pub(super) use codetether_session_location::{forget, record, resolve};
#[cfg(test)]
#[path = "location/journal_tests.rs"]
mod journal_tests;
