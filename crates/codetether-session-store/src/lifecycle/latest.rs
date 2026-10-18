//! Workspace-scoped latest-session selection using the host's ordered listing.
use super::Host;
use crate::TailLoad;
use anyhow::Result;
/// Load the first listed session after canonicalizing the workspace path.
///
/// # Arguments
/// * `workspace` — Optional directory filter.
/// * `window` — Maximum retained message and tool-use window.
/// # Returns
/// The latest matching session window.
/// # Errors
/// Directory resolution, listing, loading or absence failures.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_session_store::lifecycle::Host>() -> anyhow::Result<()> {
/// let _result = codetether_session_store::lifecycle::last_for_directory_tail::<H>(None, 100).await?;
/// # Ok(())
/// # }
/// ```
pub async fn last_for_directory_tail<H: Host>(
    workspace: Option<&std::path::Path>,
    window: usize,
) -> Result<TailLoad<H::Session>> {
    let _sessions_dir = super::sessions_dir()?;
    let canonical_workspace = workspace.map(|w| {
        w.canonicalize().unwrap_or_else(|e| {
            tracing::warn!(path = %w.display(), error = %e, "canonicalize failed; using raw path");
            w.to_path_buf()
        })
    });
    let sessions = H::list_session_ids(canonical_workspace.as_deref()).await?;
    let latest = sessions
        .first()
        .ok_or_else(|| anyhow::anyhow!("No sessions found"))?;
    super::load_tail::<H>(latest, window).await
}
