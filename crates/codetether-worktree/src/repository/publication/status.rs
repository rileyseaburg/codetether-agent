//! Repository publication status operation.
use super::*;

pub(super) async fn status(repo_path: &Path) -> Result<String> {
    git(repo_path, &["status", "--porcelain"]).await
}
