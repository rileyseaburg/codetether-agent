//! Instructions for isolation and explicit parent integration.
use crate::Handoff;
impl Handoff {
    /// Render checkout identity and the existing explicit-integration contract.
    /// # Examples
    /// See [Handoff] for construction and a guidance assertion.

    pub fn guidance(&self) -> String {
        format!(
            "Isolated child workspace: {}\nBranch: {}\nBase commit: {}\n\
             Parent checkout (do not edit): {}\n\
             Parent uncommitted changes are NOT copied. Rebase task paths onto your child workspace; \
             never write to parent paths inherited from conversation history.\n\
             Work only in this checkout. Commit only your task's files on this branch and report \
             commit IDs and verification evidence. Do not merge, push, or delete the worktree.\n\
             Integration: when your turn completes, the harness applies this branch's changes to \
             the parent checkout as uncommitted edits for the user to review. Nothing is \
             auto-merged or committed to the parent branch. The child checkout is retained for \
             review, recovery, and resume.",
            self.workspace.display(),
            self.branch,
            self.base_commit,
            self.parent_workspace.display()
        )
    }
}
