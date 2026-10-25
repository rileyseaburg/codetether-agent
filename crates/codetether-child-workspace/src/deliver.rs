//! Deliver a finished child's branch into the checkout that spawned it.
use crate::Handoff;
use codetether_tool_core::ToolResult;
use codetether_worktree::WorktreeInfo;

impl Handoff {
    /// Apply the child's work to the parent checkout as uncommitted edits.
    ///
    /// Pending child edits are first committed on the child branch only. The
    /// parent's commits, index and branch are never touched; a patch that
    /// does not apply leaves the parent untouched and fails the result.
    /// Failed child results are returned unchanged and nothing is delivered.
    ///
    /// # Arguments
    /// * `result` — Completed child turn result.
    /// # Returns
    /// The result with a delivery summary appended.
    /// # Examples
    /// Call with a [Handoff] from [crate::allocate] after a child turn completes.
    pub async fn deliver(&self, mut result: ToolResult) -> ToolResult {
        if !result.success {
            return result;
        }
        let info = WorktreeInfo {
            name: self
                .worktree
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
            path: self.worktree.clone(),
            branch: self.branch.clone(),
            active: true,
        };
        let outcome = match codetether_worktree::swarm::commit::prepare(&info, "child").await {
            Ok(()) => {
                codetether_worktree::deliver_branch(&self.branch, &self.parent_workspace).await
            }
            Err(error) => Err(error),
        };
        let (ok, summary) = match outcome {
            Ok(delivery) => (delivery.status.is_delivered(), delivery.summary()),
            Err(error) => (false, format!("Child delivery failed: {error:#}")),
        };
        result.success = ok;
        result.output = format!("{}\n\n--- Delivery Result ---\n{summary}", result.output);
        result
    }
}
