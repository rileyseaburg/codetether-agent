use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

mod git;
use git::run;

pub(super) struct Fixture {
    root: TempDir,
}

impl Fixture {
    pub(super) fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        run(root.path(), &["init", "-q"]);
        run(root.path(), &["config", "user.email", "test@example.com"]);
        run(root.path(), &["config", "user.name", "Test"]);
        fs::create_dir_all(root.path().join("ui/node_modules")).unwrap();
        fs::write(root.path().join("ui/package.json"), "{}").unwrap();
        fs::write(root.path().join(".gitignore"), "node_modules/\n").unwrap();
        run(root.path(), &["add", "."]);
        run(root.path(), &["commit", "-qm", "fixture"]);
        run(
            root.path(),
            &["worktree", "add", "-q", "worktree", "-b", "test-worktree"],
        );
        Self { root }
    }

    pub(super) fn root(&self) -> &Path {
        self.root.path()
    }

    pub(super) fn worktree(&self) -> PathBuf {
        self.root.path().join("worktree")
    }
}
