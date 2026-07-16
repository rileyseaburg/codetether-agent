use super::git::run;
use std::path::PathBuf;

pub(super) struct Fixture {
    _temp: tempfile::TempDir,
    pub(super) repo: PathBuf,
    pub(super) root: PathBuf,
}

impl Fixture {
    pub(super) fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let repo = temp.path().join("repo");
        let root = temp.path().join("worktrees");
        std::fs::create_dir(&repo).unwrap();
        run(&repo, &["init", "-q", "-b", "main"]);
        run(&repo, &["config", "user.email", "test@example.com"]);
        run(&repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("seed"), "seed").unwrap();
        run(&repo, &["add", "seed"]);
        run(&repo, &["commit", "-q", "-m", "seed"]);
        Self {
            _temp: temp,
            repo,
            root,
        }
    }

    pub(super) fn add(&self, name: &str) -> PathBuf {
        let path = self.root.join(name);
        run(
            &self.repo,
            &["worktree", "add", "-q", "-b", name, path.to_str().unwrap()],
        );
        path
    }
}
