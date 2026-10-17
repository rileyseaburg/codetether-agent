//! Shared hook precondition used by worker publication.
#[cfg(test)]
pub(super) use codetether_worktree::repository::publication::require_installed;
#[cfg(test)]
#[path = "git_hook_gate_tests.rs"]
mod tests;
