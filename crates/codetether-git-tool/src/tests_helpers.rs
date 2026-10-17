//! Shared helpers for git tool tests.

/// Run `git <args>` in `dir`, ignoring output (test setup only).
pub(super) async fn run_git(dir: &std::path::Path, args: &[&str]) {
    tokio::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .unwrap();
}

/// Create and configure a fresh temporary git repository.
pub(super) async fn init_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path();
    run_git(p, &["init"]).await;
    run_git(p, &["config", "user.email", "t@example.com"]).await;
    run_git(p, &["config", "user.name", "Tester"]).await;
    dir
}
