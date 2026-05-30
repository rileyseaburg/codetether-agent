//! Support helpers for `swarm_execute`: sub-agent tool filtering and
//! automatic per-task git worktree provisioning.
//!
//! Sub-agents must never edit the shared checkout. Any mutating task is
//! given its own isolated worktree so parallel agents can't clobber each
//! other; read-only tasks keep running against the shared directory.

use crate::swarm::subtask::SubTask;
use crate::tool::ToolRegistry;
use crate::worktree::WorktreeManager;
use std::path::PathBuf;
use std::sync::Arc;

/// Tool definitions exposed to swarm sub-agents.
///
/// Interactive and recursive tools are removed because sub-agents run
/// autonomously and must not spawn further swarms.
pub(super) fn subagent_tools() -> Vec<crate::provider::ToolDefinition> {
    ToolRegistry::new()
        .definitions()
        .into_iter()
        .filter(|t| {
            !matches!(
                t.name.as_str(),
                "question"
                    | "confirm_edit"
                    | "confirm_multiedit"
                    | "plan_enter"
                    | "plan_exit"
                    | "swarm_execute"
                    | "agent"
            )
        })
        .collect()
}

/// Create one isolated worktree per mutating task.
///
/// Returns a per-task working directory aligned by index: `Some(path)` for
/// isolated mutating tasks, `None` for read-only tasks or on creation failure
/// (the sub-agent then falls back to the shared directory).
pub(super) async fn create_worktrees(tasks: &[(String, String)]) -> Vec<Option<PathBuf>> {
    let base = std::env::current_dir()
        .map(|p| p.join(".codetether-worktrees"))
        .unwrap_or_else(|_| PathBuf::from(".codetether-worktrees"));
    let mgr = Arc::new(WorktreeManager::new(base));
    let mut dirs = Vec::with_capacity(tasks.len());
    for (name, instruction) in tasks {
        if !SubTask::new(name.clone(), instruction.clone()).needs_worktree() {
            dirs.push(None);
            continue;
        }
        let slug = format!("swarm_{}", uuid::Uuid::new_v4().simple());
        match mgr.create(&slug).await {
            Ok(wt) => {
                let _ = mgr.inject_workspace_stub(&wt.path);
                dirs.push(Some(wt.path));
            }
            Err(e) => {
                tracing::warn!(task = %name, error = %e, "swarm worktree create failed; using shared dir");
                dirs.push(None);
            }
        }
    }
    dirs
}
