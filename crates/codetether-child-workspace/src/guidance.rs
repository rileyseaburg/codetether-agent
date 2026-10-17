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
             Integration is explicit: the parent reviews the child commits and cherry-picks the \
             reported task commits into its checkout under its normal write lease. Nothing is \
             auto-merged. The child checkout is retained for review, recovery, and resume.",
            self.workspace.display(),
            self.branch,
            self.base_commit,
            self.parent_workspace.display()
        )
    }
}
