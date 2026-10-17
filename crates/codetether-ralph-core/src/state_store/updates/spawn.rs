//! Schedule an owned store operation without awaiting it.
pub(super) fn spawn_update(
    fut: impl std::future::Future<Output = anyhow::Result<()>> + Send + 'static,
) {
    tokio::spawn(async move {
        if let Err(e) = fut.await {
            tracing::warn!(target: "codetether_agent::ralph::ralph_loop::persistence", error = %e, "State store update failed");
        }
    });
}
