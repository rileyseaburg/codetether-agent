use super::Host;
/// Register, synchronize workspaces and retrieve pending tasks at startup.
/// # Arguments
/// * `host` — Native startup services.
/// # Returns
/// Success after attempting all startup services and scheduling synchronization.
/// # Errors
/// Individual startup failures are logged and do not prevent later steps.
/// # Examples
/// ```ignore
/// bootstrap_worker(&native_worker).await?;
/// ```
pub async fn bootstrap_worker<H: Host>(host: &H) -> anyhow::Result<()> {
    if let Err(error) = host.register().await {
        tracing::warn!(error = %error, "Initial worker registration failed");
    }
    if let Err(error) = host.sync_workspaces().await {
        tracing::warn!(error = %error, "Initial workspace sync failed");
    }
    if let Err(error) = host.fetch_pending().await {
        tracing::warn!(error = %error, "Initial task fetch failed");
    }
    let _workspace_sync_handle = host.start_workspace_sync();
    Ok(())
}
