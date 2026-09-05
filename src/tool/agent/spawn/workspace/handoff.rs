//! Child checkout identity and explicit, non-automatic integration handoff.

use crate::tool::ToolResult;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub(in crate::tool::agent::spawn) struct Handoff {
    pub workspace: PathBuf,
    pub worktree: PathBuf,
    pub branch: String,
    pub parent_workspace: PathBuf,
    pub base_commit: String,
}

impl Handoff {
    pub(in crate::tool::agent::spawn) fn guidance(&self) -> String {
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

    pub(in crate::tool::agent::spawn) fn attach(&self, mut result: ToolResult) -> ToolResult {
        let isolation = serde_json::json!({"mode": "worktree", "checkout": self,
            "integration": "review_then_cherry_pick", "auto_merge": false});
        if let Ok(serde_json::Value::Object(mut object)) = serde_json::from_str(&result.output) {
            object.insert("isolation".into(), isolation);
            result.output = serde_json::Value::Object(object).to_string();
        } else {
            result.output = format!("{}\n\nChild isolation: {isolation}", result.output);
        }
        result
    }
}
