//! Load a compatible durable identity for resumption.
use crate::{ClosedAgent, Host, restore, scan as manifest_scan};
use anyhow::{Result, bail};
/// Load a compatible child identity for later activation.
///
/// # Arguments
/// `owner` selects the owner scope; `target` is a display name or durable ID.
/// # Returns
/// Returns the prepared entry and its saved manifest.
/// # Errors
/// The target is missing, incompatible, collides with another owner, or cannot load.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_storage::{Host, prepare_resume};
/// # async fn example<H: Host>() -> anyhow::Result<()> {
/// let prepared = prepare_resume::<H>(Some("parent"), "reviewer").await?;
/// assert_eq!(prepared.name, "reviewer");
/// # Ok(()) }
/// ```
pub async fn prepare_resume<H: Host>(
    owner: Option<&str>,
    target: &str,
) -> Result<ClosedAgent<H::Session>> {
    let manifest = manifest_scan::find(owner, target)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Agent {target} not found"))?;
    if !manifest.supports_current_schema() {
        bail!("Agent {target} uses an unsupported manifest version");
    }
    let entry = restore::load::<H>(&manifest).await?;
    let name = manifest.name.clone();
    Ok(ClosedAgent {
        name,
        entry,
        manifest,
    })
}
