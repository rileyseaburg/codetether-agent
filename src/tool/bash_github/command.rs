//! Git command helpers for GitHub auth detection.
//!
//! This module provides the small shell probes used to detect whether a bash
//! command talks to GitHub and to read local repository metadata.
//!
//! # Examples
//!
//! ```ignore
//! assert!(needs_github_auth("gh pr create"));
//! ```

use tokio::process::Command;

/// Runs a small git probe and returns trimmed stdout on success.
///
/// The helper is used for metadata lookups that should quietly fail when the
/// current working directory is not a valid git repository.
///
/// # Examples
///
/// ```ignore
/// let root = git_stdout(Some("."), ["rev-parse", "--show-toplevel"]).await;
/// assert!(root.is_some() || root.is_none());
/// ```
pub(super) async fn git_stdout<const N: usize>(
    cwd: Option<&str>,
    args: [&str; N],
) -> Option<String> {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output().await.ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Detects whether a bash command will likely need GitHub credentials.
///
/// The matcher is intentionally conservative: it only flags obvious `gh`
/// invocations and direct GitHub API calls.
///
/// # Examples
///
/// ```ignore
/// assert!(needs_github_auth("gh api /user"));
/// assert!(!needs_github_auth("cargo test"));
/// ```
pub(super) fn needs_github_auth(command: &str) -> bool {
    let lower = command.to_ascii_lowercase();
    lower.starts_with("gh ")
        || lower.contains("\ngh ")
        || lower.contains(" gh ")
        || lower.contains("api.github.com")
        || lower.contains("github.com/repos/")
}
