use super::net_git;
use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::fs;

pub(super) struct Fixture {
    _dir: tempfile::TempDir,
    pub(super) manager: WorktreeManager,
    pub(super) info: WorktreeInfo,
}

impl Fixture {
    pub(super) fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        net_git::init(dir.path());
        let manager = WorktreeManager::for_repo(dir.path()).without_vscode_auto_open();
        let info = WorktreeInfo {
            name: "agent".into(),
            path: dir.path().into(),
            branch: "agent".into(),
            active: true,
        };
        Self {
            _dir: dir,
            manager,
            info,
        }
    }

    pub(super) fn checkout(&self, branch: &str) {
        net_git::run(&self.info.path, &["checkout", branch]);
    }

    pub(super) fn empty_commit(&self, message: &str) {
        net_git::run(
            &self.info.path,
            &["commit", "--allow-empty", "--no-gpg-sign", "-m", message],
        );
    }

    pub(super) fn write_commit(&self, content: &str, message: &str) {
        fs::write(self.info.path.join("file"), content).unwrap();
        net_git::run(
            &self.info.path,
            &["commit", "--no-gpg-sign", "-am", message],
        );
    }
}
