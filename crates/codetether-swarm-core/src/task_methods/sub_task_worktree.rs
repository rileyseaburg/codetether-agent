//! SubTask worktree operations.
use super::*;
impl SubTask {
    /// Decide whether this subtask should run in an isolated worktree.
    /// Mutating intent always wins over model-provided read-only metadata.
    /// Verification intent defaults to isolation but honors an explicit
    /// read-only workspace choice, where mutating tools remain unavailable.
    ///
    /// # Returns
    /// Whether intent and metadata require isolation.
    pub fn needs_worktree(&self) -> bool {
        match self.kind() {
            crate::task_kind::TaskKind::Mutating => true,
            crate::task_kind::TaskKind::Verification => self.needs_worktree.unwrap_or(true),
            crate::task_kind::TaskKind::ReadOnly => self.needs_worktree.unwrap_or(false),
        }
    }

    /// Whether this task should receive only read-only tools.
    ///
    /// # Returns
    /// Whether isolation is not required.
    pub fn is_read_only(&self) -> bool {
        !self.needs_worktree()
    }

    /// Whether successful completion should produce mergeable source changes.
    ///
    /// # Returns
    /// Whether the task has mutating intent.
    pub fn expects_file_changes(&self) -> bool {
        self.kind() == crate::task_kind::TaskKind::Mutating
    }

    /// Whether this task runs checks but must not change source files.
    ///
    /// # Returns
    /// Whether the task has verification intent.
    pub fn is_verification(&self) -> bool {
        self.kind() == crate::task_kind::TaskKind::Verification
    }

    pub(super) fn kind(&self) -> crate::task_kind::TaskKind {
        crate::task_kind::classify(&self.name, &self.instruction, self.specialty.as_deref())
    }
}
