use std::{fs, path::Path, process::Command};

pub(super) fn init(repo: &Path) {
    run(repo, &["init", "-b", "main"]);
    run(repo, &["config", "user.email", "test@example.com"]);
    run(repo, &["config", "user.name", "Test"]);
}

pub(super) fn write_commit(repo: &Path, content: &str, message: &str) {
    fs::write(repo.join("file"), content).unwrap();
    commit_all(repo, message);
}

pub(super) fn commit_all(repo: &Path, message: &str) {
    run(repo, &["add", "--all"]);
    run(repo, &["commit", "--no-gpg-sign", "-m", message]);
}

pub(super) fn branch_exists(repo: &Path, branch: &str) -> bool {
    Command::new("git")
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .current_dir(repo)
        .status()
        .unwrap()
        .success()
}

fn run(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
