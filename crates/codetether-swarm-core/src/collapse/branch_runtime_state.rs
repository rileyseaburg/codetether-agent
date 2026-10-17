use super::*;

/// Runtime state for a currently active branch/worktree.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let branch = codetether_swarm_core::collapse::BranchRuntimeState { subtask_id: "task".into(), branch: "work".into(), worktree_path: "worktree".into() };
/// assert_eq!(branch.branch, "work");
/// ```
pub struct BranchRuntimeState {
    pub subtask_id: String,
    pub branch: String,
    pub worktree_path: PathBuf,
}
