//! Latest-workspace session lookup with new-session fallback.
use crate::SessionSource;
use std::path::Path;
pub(super) async fn resume_or_create<S: SessionSource>(workspace_dir: &Path) -> anyhow::Result<S> {
    Ok(match S::resume_last(Some(workspace_dir)).await {
        Ok(s) => {
            tracing::info!(
                session_id = %s.id(),
                workspace = %workspace_dir.display(),
                "Continuing last workspace session"
            );
            s
        }
        Err(err) => {
            let s = S::create().await?;
            tracing::warn!(
                error = %err,
                session_id = %s.id(),
                workspace = %workspace_dir.display(),
                "Session lookup failed; created new session"
            );
            s
        }
    })
}
