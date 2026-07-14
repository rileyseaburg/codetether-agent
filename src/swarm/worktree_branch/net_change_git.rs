use std::{fs, path::Path, process::Command};

pub(super) fn init(repo: &Path) {
    run(repo, &["init", "-b", "main"]);
    run(repo, &["config", "user.email", "test@example.com"]);
    run(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("file"), "base\n").unwrap();
    run(repo, &["add", "file"]);
    run(repo, &["commit", "--no-gpg-sign", "-m", "base"]);
    run(repo, &["branch", "agent"]);
}

pub(super) fn run(repo: &Path, args: &[&str]) {
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
