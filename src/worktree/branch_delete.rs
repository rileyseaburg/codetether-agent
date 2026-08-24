//! Safe deletion of merged or tree-equivalent worktree branches.

use std::path::Path;

pub(super) async fn run(repo: &Path, branch: &str) -> bool {
    match git(repo, &["branch", "-d", branch], true).await {
        Ok(output) if output.status.success() => deleted(branch),
        Ok(_output) if equivalent(repo, branch).await => {
            match git(repo, &["branch", "-D", branch], true).await {
                Ok(forced) if forced.status.success() => deleted(branch),
                Ok(forced) => preserve(branch, &forced.stderr),
                Err(error) => failed(branch, error),
            }
        }
        Ok(output) => preserve(branch, &output.stderr),
        Err(error) => failed(branch, error),
    }
}

async fn equivalent(repo: &Path, branch: &str) -> bool {
    let head = git(repo, &["rev-parse", "HEAD^{tree}"], false).await;
    let branch = format!("{branch}^{{tree}}");
    let other = git(repo, &["rev-parse", &branch], false).await;
    matches!((head, other), (Ok(a), Ok(b)) if a.status.success() && b.status.success() && a.stdout == b.stdout)
}

async fn git(repo: &Path, args: &[&str], mutating: bool) -> anyhow::Result<std::process::Output> {
    crate::tool::git::process::output_refs(repo, args, mutating).await
}

fn deleted(branch: &str) -> bool {
    tracing::info!(branch, "Deleted merged worktree branch");
    true
}
fn preserve(branch: &str, stderr: &[u8]) -> bool {
    tracing::warn!(branch, error = %String::from_utf8_lossy(stderr), "Preserving unmerged worktree branch");
    false
}
fn failed(branch: &str, error: anyhow::Error) -> bool {
    tracing::warn!(branch, %error, "Worktree branch delete failed");
    false
}
