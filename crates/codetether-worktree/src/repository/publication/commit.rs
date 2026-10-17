//! Repository publication commit operation.
use super::*;

pub(super) async fn commit(
    repo_path: &Path,
    task_id: &str,
    provenance: Option<&codetether_provenance::ExecutionProvenance>,
) -> Result<()> {
    let message = format!("CodeTether task {task_id}");
    let effective_provenance = provenance::worker_provenance(task_id, provenance);
    let output = codetether_provenance::git_commit_with_provenance(
        repo_path,
        &message,
        Some(&effective_provenance),
    )
    .await?;
    if !output.status.success() {
        anyhow::bail!(
            "git commit failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}
