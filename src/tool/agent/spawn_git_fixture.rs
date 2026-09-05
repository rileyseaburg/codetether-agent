//! Disposable repositories keep spawn tests out of the developer checkout.

pub(super) fn fixture() -> tempfile::TempDir {
    let repo = tempfile::tempdir().unwrap();
    for args in [
        vec!["init"],
        vec!["config", "user.name", "Spawn Test"],
        vec!["config", "user.email", "spawn@example.invalid"],
        vec!["config", "commit.gpgsign", "false"],
        vec!["commit", "--allow-empty", "-m", "spawn fixture"],
    ] {
        let output = std::process::Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("fixture git");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    repo
}
