use super::state::SwarmWorktrees;
use crate::worktree::WorktreeInfo;
use std::path::PathBuf;

impl SwarmWorktrees {
    pub(in crate::tool::swarm_execute) fn dir(
        &self,
        index: usize,
    ) -> anyhow::Result<Option<PathBuf>> {
        let slot = self.infos.get(index).ok_or_else(|| {
            anyhow::anyhow!("missing swarm worktree allocation state for task {index}")
        })?;
        path(slot)
    }
}

fn path(slot: &anyhow::Result<Option<WorktreeInfo>>) -> anyhow::Result<Option<PathBuf>> {
    match slot {
        Ok(info) => Ok(info.as_ref().map(|value| value.path.clone())),
        Err(error) => anyhow::bail!("{error:#}"),
    }
}

#[test]
fn preserves_the_concrete_allocation_error() {
    let slot = Err(anyhow::anyhow!("git worktree add failed"));
    let error = path(&slot).unwrap_err();
    assert_eq!(error.to_string(), "git worktree add failed");
}
