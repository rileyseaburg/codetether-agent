use super::{fixture_git as git, fixture_result};
use crate::swarm::SubTaskResult;
use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::{fs, path::PathBuf};

pub(super) struct Fixture {
    _root: tempfile::TempDir,
    repo: PathBuf,
    pub(super) manager: WorktreeManager,
    pub(super) worktree: WorktreeInfo,
}

impl Fixture {
    pub(super) async fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let repo = root.path().join("repo");
        fs::create_dir(&repo).unwrap();
        git::init(&repo);
        git::write_commit(&repo, "base\n", "base");
        let manager =
            WorktreeManager::with_repo(root.path().join("trees"), &repo).without_vscode_auto_open();
        let worktree = manager.create("agent").await.unwrap();
        Self {
            _root: root,
            repo,
            manager,
            worktree,
        }
    }

    pub(super) fn edit_agent(&self, content: &str) {
        fs::write(self.worktree.path.join("file"), content).unwrap();
    }

    pub(super) fn commit_agent(&self, message: &str) {
        git::commit_all(&self.worktree.path, message);
    }

    pub(super) fn edit_main(&self, content: &str, message: &str) {
        git::write_commit(&self.repo, content, message);
    }

    pub(super) fn branch_exists(&self) -> bool {
        git::branch_exists(&self.repo, &self.worktree.branch)
    }

    pub(super) fn result(&self) -> SubTaskResult {
        fixture_result::create()
    }
}
