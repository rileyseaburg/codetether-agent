use anyhow::Result;
use std::path::Path;

/// Recreate `name` at exactly `workspace`, joining the workspace's server if it
/// is still running (other sessions kept it alive) or starting a fresh one.
/// # Arguments
/// * `name` - Exact session name to recreate.
/// * `workspace` - Workspace directory for the session/server.
/// * `isolation` - Existing workspace isolation policy.
/// # Returns
/// Unit after joining the existing server or observing the new server record.
/// # Errors
/// Returns invalid-name, session-creation, process-start or startup errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_mux_lifecycle::restart::start_exact("work", std::path::Path::new("."), codetether_mux_workspace::Isolation::Shared).await?;
/// # Ok(()) }
/// ```
pub async fn start_exact(
    name: &str,
    workspace: &Path,
    isolation: codetether_mux_workspace::Isolation,
) -> Result<()> {
    codetether_mux_registry::validate_name(name)?;
    if let Some(record) = crate::start_join::live_server(workspace).await {
        crate::start_join::create_session(record, name, workspace.to_path_buf()).await?;
        return Ok(());
    }
    let token = codetether_mux_server::token::generate();
    let mut process = crate::spawn::command(name, workspace, &token, isolation)?;
    let mut child = process
        .spawn()
        .map_err(|error| anyhow::anyhow!("restart mux server: {error}"))?;
    crate::startup::wait_for_record(name, &mut child).await?;
    Ok(())
}
