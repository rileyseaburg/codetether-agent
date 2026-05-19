//! Tests for Git credential helper utilities.
//!
//! These tests cover GitHub CLI delegation and query rendering without making
//! network calls to the control plane.
//!
//! # Examples
//!
//! ```ignore
//! cargo test git_credentials
//! ```

use super::gh_cli::should_delegate_to_gh_cli;
use super::gh_query::render_gh_credential_query;
use super::{GitCredentialMaterial, GitCredentialQuery, configure_repo_git_auth};
use crate::a2a::git_credentials::repo_config::run_git_config_command_with_lock_recovery;
use std::process::Command;
use std::time::Duration;

fn sample_credentials() -> GitCredentialMaterial {
    GitCredentialMaterial {
        username: "x-access-token".to_string(),
        password: "secret".to_string(),
        expires_at: None,
        token_type: "github_app".to_string(),
        host: Some("github.com".to_string()),
        path: Some("owner/repo.git".to_string()),
    }
}

#[test]
fn delegates_to_gh_for_github_host() {
    let query = GitCredentialQuery {
        host: Some("github.com".to_string()),
        ..GitCredentialQuery::default()
    };
    assert!(should_delegate_to_gh_cli(&query, &sample_credentials()));
}

#[test]
fn skips_gh_for_non_github_host() {
    let query = GitCredentialQuery {
        host: Some("gitlab.com".to_string()),
        ..GitCredentialQuery::default()
    };
    assert!(!should_delegate_to_gh_cli(&query, &sample_credentials()));
}

#[test]
fn renders_git_credential_payload_for_gh() {
    let query = GitCredentialQuery {
        protocol: Some("https".to_string()),
        ..GitCredentialQuery::default()
    };
    let rendered = render_gh_credential_query(&query, &sample_credentials());
    assert!(rendered.contains("protocol=https\n"));
    assert!(rendered.contains("host=github.com\n"));
    assert!(rendered.contains("path=owner/repo.git\n"));
    assert!(rendered.ends_with("\n\n"));
}

#[test]
fn configures_helper_outside_worktree() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo = dir.path();
    let init = Command::new("git")
        .current_dir(repo)
        .args(["init"])
        .output()
        .expect("git init should run");
    assert!(
        init.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );

    let helper = configure_repo_git_auth(repo, "ws-1").expect("configure helper");

    assert!(helper.starts_with(repo.join(".git")));
    assert!(!repo.join(".codetether-git-credential-helper").exists());

    let status = Command::new("git")
        .current_dir(repo)
        .args(["status", "--short"])
        .output()
        .expect("git status should run");
    assert!(
        status.status.success(),
        "git status failed: {}",
        String::from_utf8_lossy(&status.stderr)
    );
    assert!(!String::from_utf8_lossy(&status.stdout).contains("codetether-git-credential-helper"));
}

#[test]
fn recovers_stale_git_config_lock_before_config_retry() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo = dir.path();
    let init = Command::new("git")
        .current_dir(repo)
        .args(["init"])
        .output()
        .expect("git init should run");
    assert!(
        init.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    let lock_path = repo.join(".git/config.lock");
    std::fs::write(&lock_path, b"stale interrupted writer").expect("write stale lock");

    run_git_config_command_with_lock_recovery(
        repo,
        &["config", "--local", "codetether.workspaceId", "ws-lock"],
        Duration::ZERO,
    )
    .expect("stale config lock should be recovered");

    assert!(!lock_path.exists());
    let value = Command::new("git")
        .current_dir(repo)
        .args(["config", "--local", "--get", "codetether.workspaceId"])
        .output()
        .expect("git config get should run");
    assert!(value.status.success());
    assert_eq!(String::from_utf8_lossy(&value.stdout).trim(), "ws-lock");
}

#[test]
fn refuses_to_remove_fresh_git_config_lock() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo = dir.path();
    let init = Command::new("git")
        .current_dir(repo)
        .args(["init"])
        .output()
        .expect("git init should run");
    assert!(
        init.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&init.stderr)
    );
    let lock_path = repo.join(".git/config.lock");
    std::fs::write(&lock_path, b"active writer").expect("write fresh lock");

    let error = run_git_config_command_with_lock_recovery(
        repo,
        &["config", "--local", "codetether.workspaceId", "ws-fresh"],
        Duration::from_secs(3600),
    )
    .expect_err("fresh config lock should not be removed");

    assert!(lock_path.exists());
    assert!(
        error
            .chain()
            .any(|cause| cause.to_string().contains("too new to remove safely"))
    );
}
