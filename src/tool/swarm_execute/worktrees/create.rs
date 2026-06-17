use super::state::SwarmWorktrees;
use crate::swarm::subtask::SubTask;
use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::sync::Arc;

impl SwarmWorktrees {
    pub(in crate::tool::swarm_execute) async fn create(tasks: &[(String, String)]) -> Self {
        let repo = std::env::current_dir().unwrap_or_else(|_| ".".into());
        let mgr = Arc::new(WorktreeManager::for_repo(repo).without_vscode_auto_open());
        let mut infos = Vec::with_capacity(tasks.len());
        for (name, instruction) in tasks {
            infos.push(create_one(&mgr, name, instruction).await);
        }
        Self { mgr, infos }
    }
}

async fn create_one(
    mgr: &Arc<WorktreeManager>,
    name: &str,
    instruction: &str,
) -> Option<WorktreeInfo> {
    if !SubTask::new(name.to_string(), instruction.to_string()).needs_worktree() {
        return None;
    }
    let slug = format!("swarm_{}", uuid::Uuid::new_v4().simple());
    match mgr.create(&slug).await {
        Ok(wt) => {
            let _ = mgr.inject_workspace_stub(&wt.path);
            Some(wt)
        }
        Err(error) => {
            tracing::warn!(task = %name, error = %error, "swarm worktree create failed");
            None
        }
    }
}
