use std::path::PathBuf;

use codetether_workspace_snapshot::WorkspaceSnapshot;

pub(super) async fn capture(cwd: PathBuf) -> WorkspaceSnapshot {
    match tokio::task::spawn_blocking({
        let cwd = cwd.clone();
        move || WorkspaceSnapshot::capture(&cwd, 18)
    })
    .await
    {
        Ok(snapshot) => snapshot,
        Err(err) => {
            tracing::warn!(error = %err, "Workspace snapshot task failed");
            WorkspaceSnapshot::capture(&cwd, 18)
        }
    }
}
