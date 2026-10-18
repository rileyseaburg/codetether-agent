//! Task-intent classification for direct swarm workers.
//!
//! Use these helpers to derive read-only and change expectations from a task.
pub use crate::workspace::{working_directory, workspace};

/// Determine whether task intent forbids source changes.
/// # Arguments
/// * `name` — Task identity.
/// * `instruction` — Task instructions.
/// * `specialty` — Optional worker role.
/// * `needs_worktree` — Explicit isolation intent.
/// # Returns
/// Whether the task is read-only.
/// # Examples
/// ```no_run
/// assert!(codetether_swarm_tool::support::is_read_only("review", "Review", None, Some(false)));
/// ```
pub fn is_read_only(
    name: &str,
    instruction: &str,
    specialty: Option<&str>,
    needs_worktree: Option<bool>,
) -> bool {
    super::support_task::build(name, instruction, specialty, needs_worktree).is_read_only()
}

/// Determine whether task intent requires a source patch.
/// # Arguments
/// * `name` — Task identity.
/// * `instruction` — Task instructions.
/// * `specialty` — Optional worker role.
/// * `needs_worktree` — Explicit isolation intent.
/// # Returns
/// Whether a source patch is expected.
/// # Examples
/// ```no_run
/// assert!(!codetether_swarm_tool::support::expects_changes("review", "Review", None, Some(false)));
/// ```
pub fn expects_changes(
    name: &str,
    instruction: &str,
    specialty: Option<&str>,
    needs_worktree: Option<bool>,
) -> bool {
    super::support_task::build(name, instruction, specialty, needs_worktree).expects_file_changes()
}

#[cfg(test)]
#[path = "support_tests.rs"]
mod tests;
