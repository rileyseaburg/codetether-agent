//! Initial worker registration/bootstrap shared by both entrypoints.

use anyhow::Result;

use super::{
    WorkerContext, fetch_pending_tasks, register_worker, start_workspace_sync,
    sync_workspaces_from_server,
};

pub(super) async fn bootstrap_worker(context: &WorkerContext) -> Result<()> {
    register_current_worker(context).await?;
    if let Err(error) = sync_workspaces_from_server(
        &context.task_runtime.client,
        &context.server,
        &context.args.token,
        &context.shared_codebases,
    )
    .await
    {
        tracing::warn!(error = %error, "Initial workspace sync failed");
    }
    fetch_pending_tasks(&context.task_runtime).await?;
    let _workspace_sync_handle = start_workspace_sync(
        context.task_runtime.client.clone(),
        context.server.clone(),
        context.args.token.clone(),
        context.shared_codebases.clone(),
    );
    Ok(())
}

pub(super) async fn register_current_worker(context: &WorkerContext) -> Result<()> {
    let codebases = context.shared_codebases.lock().await.clone();
    register_worker(
        &context.task_runtime.client,
        &context.server,
        &context.args.token,
        &context.worker_id,
        &context.name,
        &codebases,
        context.args.public_url.as_deref(),
    )
    .await
}
