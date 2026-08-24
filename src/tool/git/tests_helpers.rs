//! Shared helpers for git tool tests.

#[path = "tests_scope.rs"]
mod scope;
pub(crate) use scope::scoped;

/// Run `git <args>` in `dir`, ignoring output (test setup only).
pub(super) async fn run_git(dir: &std::path::Path, args: &[&str]) {
    tokio::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .unwrap();
}

macro_rules! environment {
    ($dir:expr) => {
        let _lock = crate::approval::test_env::lock_env();
        let _env = crate::approval::test_env::ScopedEnv::data_dir_with_access(
            $dir.path(),
            crate::config::AccessMode::Full,
        );
    };
}

pub(super) use environment;

macro_rules! require_sandbox {
    () => {
        if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
            panic!("mandatory sandbox unavailable: {reason}");
        }
    };
}

pub(super) use require_sandbox;

/// Create and configure a fresh temporary git repository.
pub(super) async fn init_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path();
    run_git(p, &["init"]).await;
    run_git(p, &["config", "user.email", "t@example.com"]).await;
    run_git(p, &["config", "user.name", "Tester"]).await;
    dir
}
