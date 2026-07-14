use std::{fs, path::Path};
use tempfile::TempDir;

mod git;
use git::run;

pub(super) struct Fixture {
    root: TempDir,
}

impl Fixture {
    pub(super) fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        run(root.path(), &["init", "-q", "-b", "main"]);
        run(root.path(), &["config", "user.email", "test@example.com"]);
        run(root.path(), &["config", "user.name", "Test"]);
        fs::write(root.path().join("base.txt"), "base\n").unwrap();
        commit(root.path(), "base");
        run(root.path(), &["branch", "feature"]);
        fs::write(root.path().join("history.txt"), "same patch\n").unwrap();
        commit(root.path(), "main historical refactor");
        run(root.path(), &["checkout", "-q", "feature"]);
        fs::write(root.path().join("history.txt"), "same patch\n").unwrap();
        commit(root.path(), "feature historical refactor");
        for n in 1..=6 {
            fs::write(root.path().join(format!("live-card-{n}.ts")), "feature\n").unwrap();
        }
        commit(root.path(), "live card audio");
        run(root.path(), &["checkout", "-q", "main"]);
        Self { root }
    }

    pub(super) fn root(&self) -> &Path {
        self.root.path()
    }
}

fn commit(root: &Path, message: &str) {
    run(root, &["add", "."]);
    run(root, &["commit", "-qm", message]);
}
