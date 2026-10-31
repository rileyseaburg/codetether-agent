//! Worker capability selection independent of shared-workspace task wording.
use crate::SubTask;

/// Effective tool access and source-change expectation for a task.
/// # Examples
/// ```
/// use codetether_swarm_core::{SubTask, tool_policy::TaskAccess};
/// let access = TaskAccess::for_task(&SubTask::new("review", "Inspect code"), true);
/// assert!(!access.read_only && !access.verification);
/// assert!(!access.expects_changes);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TaskAccess {
    /// Restrict the worker to inspection tools.
    pub read_only: bool,
    /// Restrict the worker to verification tools.
    pub verification: bool,
    /// Require source changes when integrating an isolated task.
    pub expects_changes: bool,
}

impl TaskAccess {
    /// Shared workers retain editing tools without requiring unnecessary edits.
    /// # Arguments
    /// * `task` - Assignment and advisory worktree preference.
    /// * `shared_workspace` - Whether every worker can edit the parent workspace.
    /// # Returns
    /// Tool restrictions for isolated tasks, or writable access for shared tasks.
    /// # Examples
    /// ```
    /// use codetether_swarm_core::{SubTask, tool_policy::TaskAccess};
    /// let access = TaskAccess::for_task(&SubTask::new("review", "Inspect code"), true);
    /// assert!(!access.read_only && !access.verification);
    /// ```
    pub fn for_task(task: &SubTask, shared_workspace: bool) -> Self {
        Self {
            read_only: !shared_workspace && task.is_read_only(),
            verification: !shared_workspace && task.is_verification(),
            expects_changes: task.expects_file_changes(),
        }
    }
}
